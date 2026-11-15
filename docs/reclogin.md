# reclogin: One Login Screen for LevitateOS and AcornOS

Status: **documented, not started.** This file records the requirement and the design constraints for a future `reclogin`. Do not treat it as an implementation; no programming or porting is authorized by this document alone.

## The requirement

LevitateOS and AcornOS must have **the same login screen**. Today both use SDDM with different themes and behavior. `reclogin` replaces SDDM on both systems with one Levitate-owned login manager, written in Rust, presenting the same greeter, interaction model, and behavior on both OSes. Only branding differs, driven by the distro profile (LevitateOS: leaf/pine identity; AcornOS: acorn/brown identity).

Out of scope, explicitly: no agent integration, no embedded agent workflows, no extra account/recovery machinery, and no attempt to replace PAM, logind/elogind, or the desktop sessions themselves. `reclogin` is a login manager and greeter, nothing more.

## What it must do

- Show the user list (users that may log in graphically), a password prompt, and session selection (the desktop sessions installed on that machine, e.g. Plasma on LevitateOS, LXQt on AcornOS).
- Authenticate through PAM, with failed-attempt handling that does not leak whether the user exists beyond what the user list already shows, clears password material after use, and applies a delay/lockout policy on repeated failures.
- Start the selected session on a seat via the system's session manager — systemd-logind on LevitateOS, elogind on AcornOS — speaking to both through the same logind D-Bus interface, so one code path serves systemd and OpenRC systems.
- Handle the greeter lifecycle correctly: greeter runs on its own VT/session, the user session replaces it on login, and logging out returns to a fresh greeter.
- Offer the expected power actions from the login screen (shut down, restart, and sleep/hibernate where the machine supports them), authorized through logind/elogind.
- Respect the machine's keyboard layout (including switching where configured), show the current layout, and keep the on-screen keyboard available where the distro enables it.
- Behave identically in failure: if the greeter cannot start or crashes, a console (getty) login must still be reachable; a broken greeter must never lock the user out of the machine.

## Constraints from the rest of the system

- **One implementation, both inits.** No systemd-only assumptions: LevitateOS is systemd/glibc, AcornOS is OpenRC/musl with elogind. Anything that cannot be expressed over PAM + logind/elogind D-Bus does not belong in `reclogin`.
- **Sessions are data, not code.** Available sessions come from the installed session files (Wayland/X session directories); `reclogin` does not hardcode Plasma or LXQt.
- **Branding comes from the distro profile** (name, colors, logo, background), the same source the rest of the system branding uses. No per-distro forks of the greeter.
- **No agent anything.** Per the product rule recorded in the installer/recab handoff: no “ask an agent,” no picker, no embedded agent workflow in any product GUI, including this one.
- **Security is the design center.** Password handling, PAM conversation, VT switching, and session handover are the parts that must be reviewed hardest; visual polish must never weaken them.

## Open questions to settle before implementation

- Rendering stack: the greeter needs a UI toolkit that runs before any user session on both systems (Qt is the natural candidate — both desktops are Qt-based — but the choice, and how the greeter process hosts it pre-login, is undecided).
- How much theming is profile data vs. fixed design (backgrounds, accent color, logo placement, user-list style).
- Exact failure/fallback semantics: how many greeter crashes trigger what fallback, and how that is surfaced to the user.
- Whether multi-seat and remote (VNC/RDP-style) logins are in scope at all; assume not until the user says otherwise.

## What the user sees (design research, 2026-11-15)

Researched from today's shipped SDDM screens (Levitate Breeze vs Acorn Maldives QML/config), comparable greeters (SDDM/LightDM/Slick/tuigreet/Lemurs/Ly/COSMIC), and both distros' PAM + login1/elogind constraints. “OPEN” below means undecided, not promised. Status: **partial** — inventories of current screens come from packaged QML/config, not boot/screenshot verification; elogind gaps are inferred and need runtime probing.

Scope reminder: reclogin owns only the pre-login greeter on seat0/tty1. It never locks a running session and never draws a lock screen (logind only signals the desktop's own locker). No account creation/recovery machinery.

### Common chrome (every screen)

Distro wallpaper + profile branding; clock; hostname welcome text (today Acorn-style); keyboard-layout indicator/switcher shown **before** password focus (only when >1 layout is configured); live Caps-Lock warning; labelled power menu; session picker; on-screen-keyboard button at least where Levitate has one today (Acorn parity is OPEN). The UI must render arbitrary multi-line PAM info/error text in order — never assume a single password box.

### Normal login

Clickable avatar + real-name user list for listable users (uid 1000–60000, login shell, non-empty home; root/system never listed; AccountsService may enrich on Levitate, passwd fallback is mandatory, Acorn is passwd-only), last user preselected. “Other user…” manual-username entry is always available (list-only is forbidden). Password field masked with show-password toggle and Caps warning; focus starts in the password. Session picker enumerates `/usr/share/wayland-sessions` + `/usr/share/xsessions`, validates entries (dead entries are never offered), hides itself when only one valid session exists, and remembers the last session (today: Plasma (Wayland) on Levitate; LXQt + Openbox on Acorn).

### Wrong password

Same screen, in place: password cleared, user and session choices preserved, focus back to password, shake plus a red generic message (“Authentication failed”); never say which field failed. PAM delay applies; lockout/attempts text only if PAM supplies it (no `pam_faillock`/`pam_pwquality` wired today). Empty submit is handled locally (shake + inline message, no PAM call).

### Locked / expired / nologin

Whatever PAM says is rendered verbatim in place (multi-line). Expired passwords run the full multi-step conversation on the same screen (current → new ×2, errors in place) and continue to session start; hard-expired accounts and `/etc/nologin` show PAM's message instead of a dead prompt. The greeter never invents its own lockout text.

### Power actions

Labelled corner/footer menu, gated live by logind `Can*` answers: unsupported → hidden; `challenge` → polkit admin-auth flow; inhibited → confirmation. Re-queried on change, never cached. **Hibernate is never offered** on current images (`AllowHibernation=no` on both; recinit cannot resume) — this supersedes the “sleep/hibernate where the machine supports them” wording above.

### Keyboard layout

Current-layout label and switcher visible before password entry (wrong layout/Caps is the top cause of “correct password rejected”); show the effective layout rather than silently defaulting to US. Caps warning is live at focus and on toggle.

### No users / root-only

With no listable users, the list is replaced by a typed-username field plus password and a message pointing to a getty on another VT / first-user creation in the installer. Reclogin never creates accounts. Root is never in the clickable list; graphical-root policy is OPEN.

### Interaction rules

Keyboard-first throughout: focus order user/username → password → session/layout (Tab, no traps) → Login; selecting a user moves focus to the password; Enter in the password submits; Enter in manual username moves to the password. Power/session/layout controls are always labelled on screen (function-key shortcuts may exist only in addition). Wrong-password never loses user/session state. Switching away from a running session hands off to that session's own desktop locker — reclogin never draws an unlock prompt for it.

### States (summary)

Greeter-idle → authenticating → (failed: same screen, cleared password) / (extra PAM prompts: same screen) / handover to the user session → logout returns to a fresh greeter. Autologin (when configured) goes straight to handover; failure falls back to the greeter. Greeter crash: supervisor restarts it, and past a threshold the VT is freed so a getty on tty2–tty6 (or serial) is reachable — never a black-screen lockout. Live ISO: display manager stays off, tty1 is the installer (unchanged).

### What may differ vs. what must be identical

May differ (profile/branding only): distro name/logo, colours, wallpaper, background treatment — cursor theme joins this list only if confirmed (today breeze vs Adwaita). Offered sessions differ as data, not code. Plumbing differs invisibly (systemd+logind vs OpenRC+elogind; AccountsService present/absent) against the oldest common login1 denominator, probed at runtime.

Must be identical: the greeter binary and interaction model; avatar list + last-user preselection + always-available manual entry (Acorn gains the list; Levitate must not degrade to typed-only); session enumeration/validation; wrong-password, locked, and expired-password UX; layout indicator + Caps warning; power gating including hibernate-never; autologin and live-ISO DM-off behavior; getty fallback. Parity items still to settle honestly: on-screen keyboard (Levitate only today), battery indicator, hostname welcome placement, show-password toggle.

### Decisions still open for the user

1. Graphical root: forbidden vs typed-name-only (and faillock root policy).
2. Greeter crash threshold/window and how failure is surfaced, on each supervisor.
3. Empty-submit confirmation and exact generic error wording / attempts-remaining display.
4. Last-session memory: per-user vs global; same-stem session-file collision priority.
5. Autologin: keep reading the SDDM drop-in vs new reclogin config (+migration), countdown/cancel, failed-autologin fallback, relogin-on-logout.
6. On-screen keyboard required on both distros vs profile-gated (Acorn would be an addition, not parity).
7. Cursor theme as profile branding.
8. Clock: date shown?, 12/24h source, timezone/NTP indication.
9. Multi-monitor behavior, DPMS wake, brightness keys, preserving Levitate's 60s auto-fade.
10. Accessibility scope beyond OSK + Caps + announcements (a11y menu, persistence into the session).
11. Greeter/session language picker and localized PAM text.
12. Running-session list / switch-user entries vs fresh login only; power warnings with other users logged in.
13. Guest session (assumed off — confirm) and network/LDAP manual entry.
14. Non-password PAM factors (fingerprint/smartcard/FIDO2/OTP) explicitly in or out of scope.
15. NumLock initial state; single-user flow details (auto-focus, default avatar, duplicate names); busy/cancel during slow PAM; idle password clearing.
16. Toolkit choice (from §Open questions) — it decides whether Caps/OSK behaviors are inherited or hand-built; elogind parity of every `Can*`/lock call must be runtime-verified, not assumed.

## Acceptance sketch (for when this is picked up)

- On both LevitateOS and AcornOS, a fresh boot lands on the same login screen (modulo branding), the user logs into their desktop, logs out back to the greeter, and can shut down/restart from it.
- Wrong passwords are refused with delay and no extra information; the machine never sits at a dead greeter with no way to a console login.
- No SDDM remains installed or started on either system.

# reclogin: Implementation Plan

Status: **plan, written 2026-11-15 on the owner's word. All stages are written in `tools/reclogin`, including the guest account; none was built into an image or run. Stage 10 is written too: both prebuilts are published (`login-screen-86d815752d902d81`, LevitateOS; `login-screen-ffd914bc27cad91d`, AcornOS), and the switch from SDDM is written (2026-11-16): `recipes/login-screen.rhai` installs it on both OSes; nothing of it has been built into an image or run.** It builds what these documents settle and changes none of it:

- `docs/reclogin.md`: what reclogin is, the screens, the owner's decisions.
- `docs/reclogin-guest.md`: the guest account.
- `docs/reclogin-technical.md`: the parts, and what the try of 2026-11-15 showed on both OSes.
- The design canvas with the twelve screens (the look the owner accepted).

## 1. Rules for this work

- Code is written and compile-checked. Nothing is built into an image and nothing is run to prove it, unless the owner says so. The owner's VMs are tried only on the owner's word, each time.
- One branch per repository, commits with sign-off, pushed as each stage lands. No unit tests.
- Nothing of SDDM stays once reclogin replaces it.
- The main session writes the briefs, reviews every diff and commits. Agents write.

## 2. What gets made

One crate, `tools/reclogin`, with a small shared library and two programs.

| Piece | In it |
|---|---|
| Shared | The messages between the two programs. The settings file (`/etc/reclogin.conf`). The brand data. |
| `reclogind` (root) | Consoles. Opening the display and input devices and handing them over. PAM. Starting and ending sessions, Wayland and X. The login service (who is logged in, power, bringing a session to the front). Users and session files. Last session. Automatic login. The guest and its cleanup. Restarting the greeter and giving up after three crashes. |
| `reclogin-greeter` (no rights) | Setting the mode and drawing. Input and keyboard layouts. The twelve screens. The on-screen keyboard. Several monitors. Idle switch-off. Its own lock-down. |

Outside the crate:

| Where | What |
|---|---|
| `distro/builder` | Builds reclogin for each OS against that OS's libraries and publishes it as a prebuilt, the way the desktop apps are. |
| `recipes/login-screen.rhai` | Becomes the login screen of both OSes: the two programs, two PAM files, the systemd unit or OpenRC service, the default settings file, fonts, brand data, the accounts `reclogin` and `guest`. |
| `recipes/plasma.rhai` | Loses SDDM: packages, theme, unit link, the console mask. |
| `distro/variants/*` | The frame art becomes part of each variant's branding. Both default sets name `login-screen`. |
| Installer (`system-setup.sh`, recsetup) | Automatic login is written to `/etc/reclogin.conf`. |
| `testing/install-tests` | A login scenario, written now and run at the owner's build. |
| Docs and website | The login screen described as reclogin, when it is in an image. |

## 3. Stages

Each stage ends in code that compiles and is committed. "Try" names what a run in the VMs would show; it happens only on the owner's word.

| # | Stage | Model | Depends on | Try, if the owner wants one |
|---|---|---|---|---|
| 1 | **Skeleton.** The crate, the messages, the settings file, the brand data. `reclogind` takes console 7, opens the devices, starts the greeter with them, and gives everything back at exit. The greeter sets the mode and draws the normal login screen from brand data, with working fields and keys. | opus | nothing | The real login picture on both OSes, typed into. |
| 2 | **Logging in.** PAM in `reclogind` with every question and message passed to the greeter. Wrong password, expired password. Starting a Wayland session on its own console and ending it. | opus | 1 | A real login into Plasma and back out. |
| 3 | **X sessions.** Handing over the console device, starting the X server for the user, the access key, ending it. | opus | 2 | A real login into LXQt and back out. |
| 4 | **The rest of the screens.** The user list, session picker, last session, the keyboard layout switcher, Caps Lock, "Other user", no accounts, the footer. | sonnet | 1 | Each screen against the canvas. |
| 5 | **Other people and power.** Who is logged in, returning to a session, power actions with the "others are logged in" question. | sonnet | 2, 4 | Switch user; shut down from the screen. |
| 6 | **Automatic login** and the installer writing `/etc/reclogin.conf`. | sonnet | 2 | An installed system that starts straight into a desktop. |
| 7 | **The guest**, as `docs/reclogin-guest.md` says: the account, the PAM file, the memory home, the cleanup routine. | opus | 2 | A guest session; nothing left after logout or power loss. |
| 8 | **On-screen keyboard, several monitors, idle switch-off.** | sonnet | 4 | Login by touch; a second monitor. |
| 9 | **Failing safely.** Restarting the greeter, giving up after three crashes with the line on console 1, the systemd unit, the OpenRC service, the greeter's lock-down. | opus | 1 | A greeter that crashes on purpose. |
| 10 | **Shipping.** The per-OS build and prebuilt, the `login-screen` group on both OSes, SDDM out, locks and pins regenerated, the default sets, the login scenario, the texts. | opus (builder), sonnet (recipes, texts) | 1 to 9 written | The owner's build. Written 2026-11-16 (both prebuilts published); not built into an image or run. |

Stages 2, 4 and 9 start together once stage 1 is in, because stage 1 fixes the messages both programs use. Stage 10's builder part can also be written from stage 1 on.

## 4. What is checked, and when

- **Every stage:** it compiles, and the diff is read by the main session.
- **Stages 2, 3, 7 and 9** are the ones where a mistake would not show by reading: passwords, sessions, the guest's cleanup, the fallback. Each gets a second, independent read by another agent before it is committed.
- **At the owner's build:** the login scenario, and the canvas compared with the real screen on both OSes.

## 5. What this plan does not include

- A screen reader, fingerprint or other sign-in methods beyond what PAM shows by itself, user pictures, a language picker, remote or multi-seat logins. These are out of the first version by the owner's decisions.
- Finding out why `/var/run` is a plain folder on the LevitateOS image. reclogin does not depend on it; it is a separate defect.

## 6. Risks carried into the work

- **Never seen:** the PAM modules of both systems under reclogin, a full Plasma or LXQt session started by reclogin, real graphics hardware, several monitors, touch input. The try covered the layer beneath each of these.
- **The on-screen keyboard and the lock-down** are the two parts with nothing to copy from.
- **Machines without a kernel display driver** cannot show the login screen and fall to the text logins. How many such machines the two kernels leave is not known.

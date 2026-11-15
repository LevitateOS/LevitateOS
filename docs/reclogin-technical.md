# reclogin: Technical Design

Status: **design awaiting the owner's verdict. Nothing is built. Its five assumptions were tried on both OSes on 2026-11-15 and held (section 10).** It turns the decisions in `docs/reclogin.md` and `docs/reclogin-guest.md` into parts, and names what the first implementation step has to show before the rest is worth writing. It authorizes no programming by itself.

Facts it rests on, read in the source on 2026-11-15:

- Today both OSes start SDDM from a group recipe: `recipes/plasma.rhai` (LevitateOS: `display-manager.service`, and `getty@tty1` masked) and `recipes/login-screen.rhai` (AcornOS: the `sddm` OpenRC service).
- LevitateOS's desktop is a Wayland session. AcornOS's LXQt is an X session on the XLibre server, which `recipes/xlibre.rhai` installs with its setuid wrapper.
- The installer writes automatic login as an SDDM file (`distro/builder/guest/installed/system-setup.sh`, `autologin`).
- `libinput`, `libxkbcommon`, `xkeyboard-config` and `libdrm` are in `recipes/desktop-libs.rhai` on both OSes. PAM is in the base.
- `tools/reclogin` is an empty program (25 lines).

## 1. Two programs

reclogin is two programs with different rights.

| Program | Runs as | Does |
|---|---|---|
| `reclogind` | root | Owns the login console. Opens the display and input devices. Talks to PAM. Starts and ends desktops. Asks logind for power actions and for who is logged in. Runs the guest cleanup. Counts crashes. |
| `reclogin-greeter` | the account `reclogin`, with no rights | Draws the screen, reads the keyboard, pointer and touch, and shows what `reclogind` tells it. |

Why two: drawing, fonts, images and raw input are where a bug is most likely. That code must not be able to read the password file or start a root program. The greeter gets its devices as already-open handles from `reclogind`. It cannot open anything itself: it runs with no network, a read-only view of its own data folder, and no way to gain rights.

They talk over one private socket, in lines of JSON:

- `reclogind` to greeter: the users, who is logged in, the sessions, the keyboard layouts, which power actions are allowed, the brand, and each question or message PAM has.
- Greeter to `reclogind`: the chosen user, an answer to a PAM question, the chosen session, a power action, "Guest", "return to this user".

A typed password crosses that socket once. Both sides overwrite their copy as soon as PAM has it. The greeter never learns whether a user exists beyond the list it was given.

## 2. Which console is whose

**Change from the research in `docs/reclogin.md`, which put the login screen on the first console.** The login screen takes **console 7**. Consoles 1 to 6 keep their text logins, untouched, on both OSes.

Why:

- AcornOS's text logins come from the base's inittab, which a group recipe cannot change. A login screen on console 1 would fight the text login there. On console 7 nothing has to be masked on either OS.
- A text login is then always one key press away (Ctrl+Alt+F1 to F6), which is the rule "a broken login screen never locks you out".
- Each desktop gets its own console after that (8, 9, ...). Returning to a logged-in user is a switch to their console.

At start `reclogind` switches the display to console 7. `recipes/plasma.rhai` stops masking `getty@tty1`.

## 3. Drawing and input

No X server, no Wayland compositor, no Qt, no GPU driver stack.

- **Display:** `reclogind` opens the kernel's display device and hands it to the greeter. The greeter sets the mode and draws into plain memory buffers that the kernel shows. Drawing is done on the processor: the screen is still most of the time, so this is enough and needs no Mesa.
- **Drawing library:** pure Rust (`tiny-skia` for shapes and images, `cosmic-text` for text with proper shaping and fallback fonts).
- **Input:** `libinput`, the same library the desktops use, so touchpads, mice and touchscreens behave as people expect. It asks for each device, and that request goes to `reclogind`, which opens it.
- **Keyboard layouts:** `libxkbcommon` with the layouts the system is set to. The screen shows the layout in use and switches between the configured ones.
- **Size:** the design is laid out for 1920×1080 and scaled as a whole to the real screen, using the screen's physical size where the display reports it.
- **Several monitors:** the login is on the primary one; the others show the background and the mark. Plugging a monitor in or out redraws.
- **Idle:** the display is switched off after the idle time and wakes on any input. A typed password is cleared after a minute without input.
- **On-screen keyboard:** a part of the greeter. It types only into the greeter's own fields.

When the display leaves console 7 (someone switches consoles, or a desktop starts), `reclogind` tells the greeter to stop and gives the display up. When console 7 comes back it takes the display again and the greeter redraws.

## 4. Checking who you are

`reclogind` runs the PAM conversation under its own PAM service, `reclogin`, built on the same shared rules the text login uses, so a password that works there works here.

- Every question and message PAM has is passed to the greeter and shown in order, word for word. This is what makes an expired password, an account lock or a method an administrator added work without special code.
- An empty password is refused by the greeter without calling PAM.
- Root is refused when chosen from the list (it is never in the list) and accepted when typed, as decided.
- The guest uses its own service, `reclogin-guest`, as `docs/reclogin-guest.md` describes.

## 5. Starting a desktop

After PAM says yes, `reclogind` opens the PAM session. That is what registers the login with logind on LevitateOS and elogind on AcornOS, and gives the person their runtime folder. Then it starts the chosen session on a new console, as that user, through the user's own login shell so their profile is read.

The session files in `/usr/share/wayland-sessions` and `/usr/share/xsessions` say what to run. reclogin hardcodes no desktop.

- **Wayland session (Plasma):** run the session's command. The desktop's compositor takes the display from logind by itself.
- **X session (LXQt on AcornOS):** reclogin hands the session's console device to the user, starts the X server for that user on it with the console as its input, waits until it says which display number it took, writes the access key for it, and then runs the session's command. When the session ends, reclogin stops the X server. This replaces what `startx` and SDDM did, in about one small module, with no extra program to ship.
- **Session bus:** where logind does not provide one (OpenRC), the session is started under `dbus-run-session`.

When the desktop exits, `reclogind` closes the PAM session, ends what is left of that login through logind, switches back to console 7 and starts a fresh greeter.

## 6. The other things on the screen

- **Who is logged in:** asked from logind. A logged-in user is marked. Choosing one asks logind to bring their session to the front; their desktop's own lock screen does the rest.
- **Power:** each action is asked from logind first ("may I?"). What is not possible is not shown. If other people are logged in, the greeter asks before it goes ahead. Hibernate is never offered.
- **Last session:** remembered per user in `/var/lib/reclogin/`.
- **Automatic login:** `/etc/reclogin.conf` names a user, or the guest. `reclogind` starts that session at boot without showing the greeter. If it fails, the greeter appears. After a logout the greeter appears.
- **Settings file:** `/etc/reclogin.conf`, lines of `key = value`: `autologin`, `autologin_session`, `guest`, and the idle time. Nothing else is configurable.

## 7. When it fails

- `reclogind` restarts the greeter when it exits unexpectedly.
- After three such exits within a minute it stops trying: it writes one line to console 1 ("The login screen stopped: it failed to start three times within a minute."), switches the display to console 1, where a text login is already waiting, and exits. The details are in the system log.
- `reclogind` itself is started by the init system: a systemd unit on LevitateOS (aliased as `display-manager.service`) and an OpenRC service on AcornOS. Neither restarts it endlessly: a `reclogind` that fails at once leaves the machine on its text logins.

## 8. The look

The greeter draws what the design canvas shows. What differs per OS is data, read at start:

- name, mark, and the dark palette, from the same `distro.toml` values the website and the desktops use;
- the frame art as the background.

The two typefaces (Newsreader and Instrument Sans, both under the Open Font License) are kept in the reclogin repository with their licence and installed with it. The frame art becomes part of each variant's branding.

## 9. Where it ships

- **Repository:** `tools/reclogin`, one crate, two programs.
- **Built for each OS** against that OS's own libraries, the way the desktop apps and the X server are, and published as a prebuilt. The handoff requires the prebuilt before it goes into any image.
- **Installed by the group recipe `login-screen`**, on both OSes, pinning that prebuilt. It carries the two programs, the two PAM files, the unit or service, the default settings file, the fonts, the brand data, and the accounts `reclogin` and `guest`.
- **A user can remove it** like any group. The machine then starts to its text logins, where a desktop can still be started by hand.
- **SDDM leaves both OSes:** out of `recipes/plasma.rhai` (packages, theme, unit link, the console mask) and out of `recipes/login-screen.rhai`. The locks and pins are regenerated.
- **Installer:** the automatic-login step writes `/etc/reclogin.conf` instead of the SDDM file.
- **Live medium:** unchanged. It has no login screen.

## 10. What the first try showed (2026-11-15)

A small throwaway program (`tools/reclogin/try`) was run as root in the owner's two installed VMs: LevitateOS (kernel 7.3.4, systemd-logind 262, Plasma on Wayland under SDDM) and AcornOS (kernel 6.18.63, elogind 255, LXQt on X under SDDM). Both have a virtio display at 1280×800. Screenshots were taken from the host and keys were sent from the host.

| What had to be shown | LevitateOS | AcornOS |
|---|---|---|
| 1. A program with no rights (uid 65534) draws the login picture straight to the display it was handed, with no X server and no compositor. Opening the display by itself was refused, as it must be. | yes | yes |
| 2. It reads keys and the pointer through input devices opened for it. | yes (7 of 7 keys) | yes (7 of 7 keys) |
| 3. A desktop started by us on another console takes the display, and the first console gets it back afterwards. | yes: a second Plasma compositor with a terminal, on console 1 | see 4 |
| 4. The X server, started by us as the user without root rights, in a session we registered. | not applicable | yes: XLibre with a window manager and a terminal, on console 8 |
| 5. The two login services answer alike. | power questions, the session list, session and seat properties, making a session, and bringing a session to the front all gave the same answers | same |

The running desktop came back cleanly each time on both systems.

Drawing on the processor: the still picture took 28 ms (LevitateOS) and 107 ms (AcornOS) once; a full redraw took 9 to 12 ms on average and 21 to 51 ms at worst. That is at 1280×800 in a VM. It was not measured at larger sizes or on real hardware.

What the try taught, to be built in:

- **The system bus is at `/run/dbus/system_bus_socket`.** reclogin must use that path. On LevitateOS `/var/run` is a plain folder, not a link to `/run`, so the usual path does not exist there. That is a defect of the image in its own right.
- **The console device has to be handed to the user before the X server starts.** Without that the X server stops with "Cannot open virtual console". A login program does this; `reclogind` must too, and take it back at logout.
- **The X server's setuid wrapper only starts when its input is the console.**
- **XLibre refuses the old `-noreset` option.**
- **A session can only be made from outside every existing session.** `reclogind` is started by the init system, so this holds for it; it mattered for the try, which was started from a desktop terminal.
- **Free consoles differ:** console 1 on LevitateOS today (its text login is switched off by the Plasma group), console 8 on AcornOS. Section 2's plan (login screen on 7, desktops on the next free one) does not depend on which one is free.

What the try did not cover:

- **PAM.** The sessions were registered with the login service directly, which is the call its PAM module makes. The password conversation and the PAM modules themselves were not run.
- **A full Plasma or LXQt session.** A compositor with a terminal and a window manager with a terminal stood in for them.
- **Real hardware**, graphics cards with their own drivers, several monitors, touchscreens and touchpads.
- **The greeter's lock-down** (no network, read-only view), switching consoles by key while the greeter is shown, and the idle switch-off.

Other risks, unchanged:

- **Machines whose display has no kernel driver.** The greeter needs the kernel to offer a display device. Which firmware displays the two kernels cover was not checked.
- **Screen readers** are not in the first version.
- **A locked-down greeter needs kernel features** (Landlock, namespaces). Both kernels are expected to have them; not checked.

## 11. Choices made here that the owner may change

1. The login screen on console 7, text logins left on 1 to 6 (section 2).
2. Drawing on the processor, not the graphics card (section 3).
3. User pictures are not shown in the first version; the list shows initials, as drawn. This avoids reading files from people's homes as root.
4. The fonts live in the reclogin repository (section 8).
5. Removing the `login-screen` group is allowed and leaves text logins (section 9).

## 12. Order of work, once approved

1. `reclogind`: consoles, devices, the socket, PAM, starting and ending a Wayland session.
2. The greeter: the normal login, wrong password, PAM messages, the footer.
3. X sessions.
4. Logged-in users, power, last session, automatic login.
5. The guest, as `docs/reclogin-guest.md` describes.
6. The on-screen keyboard and several monitors.
7. Crash handling and the two init services.
8. The prebuilt, the `login-screen` group on both OSes, SDDM out, the installer's setting.

These steps are written without building images. What they are worth is only known when the owner builds.

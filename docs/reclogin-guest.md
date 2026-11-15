# reclogin: the Guest Account

Status: **design awaiting the owner's verdict. Nothing is built.** No code, recipe, PAM file or account exists for it. This document settles decision 10 of `docs/reclogin.md` ("Guest session: yes … how that account is made and removed is to be designed") on paper only.

The login screen already shows the entry in its visual draft: **Guest**, with the line **"Nothing is kept after logout"**. This document says what has to be true for that line to be honest, on LevitateOS and on AcornOS, with one reclogin program.

## Summary

- The guest is **one fixed account**, `guest`, uid and gid **60600**, made when reclogin is installed. It is never created or deleted at login.
- Its home is a **fresh memory filesystem** (tmpfs) at `/run/reclogin/guest`, mounted at each guest login and unmounted at logout. It is never on a disk.
- reclogin runs **one cleanup routine** at boot, before every guest login and after every guest logout. A guest login only starts when the cleanup has finished completely.
- The guest **has no password and is locked** (`!*`). Only reclogin can start its session, through its own PAM service `reclogin-guest`. Console, ssh and `su` refuse it.
- **One guest at a time.** It may run next to logged-in users. It may use sound, printing, removable drives and existing network connections. It has no admin rights.
- An administrator can switch the entry off with one line in reclogin's settings file.

## Facts this design rests on

Read in the code on 2026-11-15:

- **The booted root is locked on an A/B installation** (`docs/product-handoff.md` §2.2). `/etc` is part of the root. So nothing may write `/etc/passwd`, `/etc/shadow` or `/etc/group` at login time. A mutable installation has one writable root, but the design must work on both.
- **Homes live under `/var/home`.** The image's `/home` is a symlink to `var/home` (`distro/builder/os-recipes/installed-system.rhai`, `homes`). On A/B, `/var` is the shared STATE partition; `recsetup` creates `/var/home` there (`tools/recstrap/src/bin/recsetup/plan.rs`). `/var` is on disk and survives power loss.
- **Accounts come from sysusers lines.** Recipe turns a change's `usr/lib/sysusers.d/*.conf` lines into accounts, the same way the image builder does. A user it makes gets the shadow password `!*`. An account that already exists is kept as it is. (`tools/recipe/src/accounts.rs`.) Explicit ids (`u name 60600:60600 …`) are supported there.
- **Installation makes the people.** `recsetup` installs the default software set through Recipe first, then creates users with the setup program (`distro/builder/guest/installed/system-setup.sh`, `user`): `useradd -m` on LevitateOS, busybox `adduser -D` on AcornOS. Users get the admin group `wheel` and the desktop groups `video input audio netdev plugdev lpadmin scanner camera` (`distro/profile/src/lib.rs`, `ADMIN_GROUP`, `DESKTOP_SESSION_GROUPS`).
- **Admin rights** are `wheel` with sudo on LevitateOS and doas on AcornOS (`distro/profile/src/lib.rs`, `admin_rule`). `recipe --system` needs root (`tools/recipe/README.md`).
- **Seats and sessions:** systemd-logind on LevitateOS; elogind and polkit as OpenRC services on AcornOS (`installed-system.rhai`, `seat_services`; `distro/variants/acorn/distro.toml` lists `elogind`, `polkit-elogind`). LevitateOS ships `systemd-pam` for `pam_systemd.so` (`distro/variants/levitate/distro.toml`).
- **PAM differs under the surface.** LevitateOS writes `system-auth`, `password-auth` and `postlogin` (`distro/builder/os-recipes/fedora-rpm-package-effects.rhai`). AcornOS has `linux-pam` in its base lock (`distro/builder/os-recipes/sources/acorn-apk-lock.rhai`), but its console login is busybox `getty` and `login` (`installed-system.rhai`, `openrc_inittab`), which read `/etc/shadow` themselves, not through PAM. Its sshd is `openssh-server`, not the PAM build. So "refuse the guest" cannot rely on PAM alone.
- **Today's autologin** is an SDDM file written by the setup program (`system-setup.sh`, `autologin`). It goes away with SDDM (reclogin decision 2).
- **Name clash to avoid:** `recguest` is already taken. It is our QEMU guest agent (`tools/recguest`, `installed-system.rhai`, `guest_agent`). The guest account is part of reclogin. It is not a new tool and must not be called recguest.

## 1. What the guest account is

**Recommended: one fixed account that exists from the moment reclogin is installed, and whose home is emptied at every logout.**

| Field | Value |
| --- | --- |
| Name | `guest` |
| Full name (GECOS) | `Guest` (the screen shows the translated word, not this field) |
| uid / gid | `60600` / `60600`, its own group `guest` |
| Home | `/run/reclogin/guest` |
| Shell | `/bin/bash` |
| Password | `!*` (locked; no password exists) |
| Supplementary groups | none |

Why fixed:

- The booted root is locked on A/B. An account made at login would have to write `/etc/passwd` and `/etc/shadow` in a root that may not be written.
- The other way around that is a name-service module that invents the account at run time. glibc can load such modules; musl cannot load any. AcornOS would need a different mechanism, which breaks "one code path".
- A fixed uid makes ending the guest's processes exact: every process with uid 60600 belongs to the guest and nobody else.

Why these numbers:

- reclogin lists users with uid 1000–60000 (`docs/reclogin.md`, "Normal login"). 60600 is outside, so the guest never appears as an ordinary user. reclogin draws the Guest entry from its own setting.
- `useradd` and busybox `adduser` hand out uids only up to 60000. They will never give 60600 to a person.
- systemd's published uid table reserves 60001–60513 and 61184–65519 and leaves 60578–61183 unused. 60600 sits in the unused band. Nothing on AcornOS claims it. The owner may pick another number in that band.

Why bash: a terminal opened inside the guest desktop starts the account's shell. With `nologin` it would close at once. Logins outside reclogin are refused by other means (point 4), not by the shell.

Rejected:

- **An account made at each guest login and deleted at logout.** Impossible on a locked root; needs a glibc-only module or a writable `/etc`.
- **A uid inside 1000–60000.** It would show up in the user list and in other programs' user pickers, and `useradd` could collide with it.
- **A system uid below 1000.** Desktops and logind treat those as services, not people (no per-user journal, other defaults).

What the guest may and may not do:

| Thing | Guest | How |
| --- | --- | --- |
| Become root (sudo, doas, admin prompts) | **No** | not in `wheel`; polkit rule below |
| Install software for everyone (`recipe --system`, the Recipe GUI's system scope) | **No** | needs root; polkit rule denies the Recipe GUI's helper action |
| Install software "for me" | Yes, into its home; gone at logout | Recipe's user store is `~/.local/share/recipe` (`tools/recipe/README.md`) |
| Read other people's homes | **No** | homes must be mode 0700 (see risk R1 for AcornOS) |
| Network | Yes, existing connections only | polkit denies NetworkManager's `settings.modify.*` for the guest |
| Removable drives | Yes: mount, unmount, eject | udisks2 defaults for the active session; everything else in udisks2 denied |
| Printing | Yes, to configured printers | CUPS lets any local user print; adding printers needs `lpadmin` |
| Sound, camera, scanner, graphics, input | Yes, while its session is the active one | logind/elogind give device access to the active session on the seat ("uaccess"); no device groups |
| Shut down, restart, suspend | As any user | logind/elogind defaults |
| Change time, language, host name, users, firmware | **No** | polkit denies |
| Keep anything running after logout (lingering, cron) | **No** | polkit denies lingering; cleanup removes crontab and linger files |

The guest gets **no** device groups on purpose. `input` would let a guest process read every keyboard on the machine, also while another user's session is in front. `lpadmin` manages printers. `video`, `audio`, `camera`, `scanner`, `plugdev` give device access that logind/elogind already grant to the active session and take away when the session is switched away.

The polkit rule is one file, `/usr/share/polkit-1/rules.d/50-reclogin-guest.rules`. polkit reads it the same way on both systems. It returns "no" for the guest on: `org.freedesktop.NetworkManager.settings.modify.*`, every `org.freedesktop.udisks2.*` action except mounting, unmounting, ejecting and powering off a removable drive, `org.freedesktop.login1.set-self-linger` and `set-user-linger`, `org.freedesktop.accounts.*`, `org.freedesktop.timedate1.*`, `org.freedesktop.locale1.*`, `org.freedesktop.hostname1.*`, firmware updates, and the Recipe/recab GUI helper actions. The goal is also that **no administrator password is ever asked for inside a guest session**: the guest controls every program in that session, so an admin who types a password there may give it away. Whether polkit's admin rule can name no administrator for a guest subject is to be confirmed against polkit's documentation when this is written; the deny list holds either way.

## 2. Where the home lives and how it is wiped

**Recommended: a new tmpfs mounted at `/run/reclogin/guest` for each guest login, and unmounted at logout.**

At login reclogin:

1. creates `/run/reclogin` (root, 0755) and `/run/reclogin/guest` (root, 0000);
2. mounts a tmpfs there with `size=<limit>,mode=0700,uid=60600,gid=60600,nosuid,nodev`;
3. checks that the mount is a tmpfs and empty (statfs and a directory read), and refuses the guest login if not;
4. copies `/etc/skel`, then the guest defaults tree `/usr/share/reclogin/guest-home/` (point 5) into it, owned by the guest.

At logout it ends the guest's processes (point 3), then unmounts the tmpfs. Unmounting a tmpfs frees its contents at once. Nothing has to be deleted file by file.

Why `/run` and not `/var/home/guest`:

- `/run` is memory on both systems and is empty at every boot. Even if the mount failed and a guest wrote into the bare directory, it would still be memory.
- The bare directory is root-owned with mode 0000, so a missing mount means the guest cannot write at all. reclogin also refuses to start the session in that case (step 3).
- `/var/home` is on disk (the STATE partition on A/B). Anything there survives power loss.

Power loss and crashes:

- **Power loss:** the home was in memory; it is gone. `/run` starts empty. Files the guest left in shared places on disk (`/var/tmp`, a crontab) are removed by the cleanup reclogin runs when it starts at boot, before it shows the Guest entry.
- **reclogin crashes while a guest is logged in:** when reclogin starts again it asks logind/elogind for sessions. If the guest has a live session, reclogin leaves it alone and marks Guest as logged in. If not, it runs the cleanup.
- **A wipe that did not finish** (a process stuck in the kernel, an unmount that failed): the cleanup routine is idempotent and runs again before every guest login. The Guest entry starts nothing until a cleanup has finished completely. Until then choosing Guest shows: "The guest session is still being cleared. Try again in a moment." If an unmount stays busy, reclogin detaches it (lazy unmount). The old contents are then unreachable by path and freed when the last stuck reference goes. The next guest always gets a new, empty tmpfs.
- **A guest session can only begin through reclogin.** So nothing a crash left behind can reach the next guest without passing the cleanup first.

Size limit: **a quarter of the machine's memory, and at most 4 GiB.** A tmpfs only uses memory for what is stored. The cap keeps a guest from pushing the whole machine into swap or the out-of-memory killer. A download larger than that fails with "No space left on device". This is the one user-visible cost of a memory home.

Rejected:

- **A directory on disk deleted at logout.** A crash or power loss leaves the files on disk. The deletion is a recursive walk that can be raced or interrupted. Deleted files stay readable on the disk's free blocks.
- **A disk directory with an encryption key thrown away at logout.** Works, but needs per-directory encryption support in the filesystem and key handling in reclogin. Too much machinery for a guest.
- **A tmpfs under `/var/home`.** Same wipe, but the bare directory underneath is on disk, so a failed mount writes to disk.
- **Wiping at the next login instead of at logout.** The files would sit on the machine between sessions, readable by root and present on a stolen disk.

## 3. One code path on both systems

reclogin uses only PAM and the `org.freedesktop.login1` D-Bus interface, which systemd-logind and elogind both provide. Everything else in this design is plain Linux: mounts, `/proc`, signals.

### Ending the guest's processes at logout

logind's `KillUserProcesses` setting is off by default on both and is configuration, not something reclogin may rely on. So reclogin ends the guest's processes itself, the same way on both:

1. The guest's session leader exits (the user logged out) and reclogin closes the PAM session.
2. reclogin calls `TerminateUser(60600)` on `org.freedesktop.login1.Manager`. On LevitateOS this also stops the guest's `systemd --user`. Errors are ignored; this step is a courtesy.
3. **The uid sweep, which is the guarantee:** reclogin forks a helper that drops to uid 60600 and calls `kill(-1, SIGKILL)`. That kills every process the guest owns, wherever it hides, including ones that double-forked out of their session. The helper repeats until `/proc` shows no process with uid 60600, for up to ten seconds.
4. Limits make the sweep win against a fork bomb: the guest PAM service applies `pam_limits` with `/etc/security/limits.d/50-reclogin-guest.conf` (`guest hard nproc 2048`).

This needs no cgroups, no systemd and no elogind-specific call. It is exact because uid 60600 is only ever the guest, and there is only one guest at a time.

### The cleanup routine

One routine, run at reclogin start, before each guest login and after each guest logout. Each step is safe to repeat.

1. If logind/elogind lists a session of user 60600, stop here (the guest is logged in).
2. `TerminateUser(60600)`, then the uid sweep.
3. Unmount everything under `/run/reclogin/guest` and `/run/media/guest` (drives the guest mounted), then the home. Remove the directories.
4. Delete the guest's files in the shared writable places: everything owned by uid 60600 under `/tmp`, `/var/tmp`, `/dev/shm` and `/dev/mqueue`. The walk never follows symlinks and stays on one filesystem.
5. Delete the guest's named leftovers if present: `/var/spool/cron/crontabs/guest` and `/var/spool/cron/guest` (busybox crond on AcornOS has a setuid `crontab`), `/var/lib/systemd/linger/guest`, `/var/lib/elogind/linger/guest`, `/var/lib/AccountsService/users/guest`, `/var/lib/AccountsService/icons/guest`, `/run/user/60600`.
6. Remove System V shared memory, semaphores and message queues owned by uid 60600 (listed in `/proc/sysvipc/`).
7. Check: no process, no mount, nothing left from steps 4 and 5. Only then is Guest available.

The list in steps 4 and 5 is owned by reclogin, in one place in its code. A new place where a guest could leave something is one new line there.

### The read-only root on A/B

Nothing in this design writes to `/etc` or `/usr` at run time.

- **Must already be in the root** (written when reclogin is installed, so present in both roots on A/B): the account lines in `/etc/passwd`, `/etc/group`, `/etc/shadow`; the PAM service `/etc/pam.d/reclogin-guest`; `/etc/security/limits.d/50-reclogin-guest.conf`; the sshd drop-in (point 4); the polkit rule; the guest defaults tree `/usr/share/reclogin/guest-home/`; reclogin's settings file.
- **Lives under `/run`:** the home and its mount point, `/run/user/60600`, mounted drives. All of it is memory and empty at boot.
- **Touched under `/var`:** only by the cleanup, which deletes guest leftovers. Nothing of the guest's is meant to live there.

On a mutable installation the same files exist in the one root. Nothing differs.

Rejected:

- **logind's `KillUserProcesses=yes`.** A systemd and elogind setting, and it would change behaviour for every user, not just the guest.
- **Killing the guest's cgroup.** systemd and elogind lay out cgroups differently. That is a systemd-shaped feature.
- **`pam_namespace` for a private `/tmp`.** Would avoid step 4, but adds a mount-namespace setup that must be checked on both PAM builds, for a sweep that is a few lines.

## 4. Authentication

**Recommended: a separate PAM service, `reclogin-guest`, that accepts only the user `guest` without a password. The account itself stays locked, so every other way in fails.**

`/etc/pam.d/reclogin-guest`, the same file on both systems:

```
#%PAM-1.0
# reclogin's guest session. No password. Only the guest account. Only reclogin
# starts this service, and only when the Guest entry was chosen.
auth      requisite  pam_succeed_if.so quiet user = guest
auth      required   pam_permit.so
account   requisite  pam_succeed_if.so quiet user = guest
account   required   pam_permit.so
password  required   pam_deny.so
session   required   pam_limits.so
session   required   pam_unix.so
-session  optional   pam_systemd.so
-session  optional   pam_elogind.so
```

The two last lines let one file serve both systems: the leading `-` tells PAM to skip a module that is not installed. LevitateOS has `pam_systemd`, AcornOS has `pam_elogind`. Because the lines are optional, reclogin checks after `pam_open_session` that logind/elogind now has a session for the guest on seat0. If not, it closes everything and shows the guest as unavailable. (That `pam_succeed_if`, `pam_permit`, `pam_limits` and `pam_unix` are all in AcornOS's `linux-pam` package is to be confirmed by reading the package's file list when this is written.)

What reclogin does:

- The **Guest entry** calls `pam_start("reclogin-guest", "guest")`, then authenticate, account, set credentials, open session, and starts the chosen desktop session as uid 60600. After PAM it is the same path as a real user.
- Before that it checks the account is really the guest: uid 60600, home `/run/reclogin/guest`, shadow password locked. If a machine has some other account called `guest` (an old one, which `accounts.rs` would keep as it is), reclogin hides the Guest entry and logs why. It never mounts a tmpfs over a real person's home.
- **"Other user…"** with the name `guest` goes through reclogin's normal PAM service like any typed name, so it fails like any wrong password. reclogin also refuses that name in the typed field before calling PAM, so typing never starts a guest session.

Why real accounts are not weakened:

- The guest's rule lives only in `reclogin-guest`. `system-auth`, `password-auth`, `login`, `sshd`, `su` and the desktop lockers' services are not touched.
- No account has an empty password. The guest's shadow field is `!*`, which matches no password at all.
- Another program could open the `reclogin-guest` service too. Run by an ordinary user, that gains nothing: PAM saying "guest is authenticated" gives an unprivileged process no new rights. Run by root, it gains nothing root does not already have.

How other logins as guest are refused:

| Way in | LevitateOS | AcornOS |
| --- | --- | --- |
| Console | util-linux `login` → PAM `login` → `pam_unix` cannot match `!*` | busybox `login` reads `/etc/shadow`, cannot match `!*` |
| ssh, password | `pam_unix` cannot match `!*` | sshd reads `/etc/shadow`, cannot match `!*` |
| ssh, key | **refused by `DenyUsers guest`** | **refused by `DenyUsers guest`** |
| `su guest` by a user | needs the guest's password, which does not exist | same |
| `su guest` by root | allowed, as root may do anything | same |

The key case matters: the guest owns its home while logged in and could write `~/.ssh/authorized_keys`. OpenSSH with PAM does not check for a locked password on key logins. So reclogin's installed tree carries `/etc/ssh/sshd_config.d/50-reclogin-guest.conf` with `DenyUsers guest`. Both OpenSSH builds should read `sshd_config.d`; that AcornOS's packaged `sshd_config` includes it is to be confirmed by reading the package when this is written (if not, the line goes into the main file the base already owns).

Rejected:

- **An empty password on the guest account.** Every console, ssh and `su` would accept it, on AcornOS without PAM even being asked.
- **A `pam_succeed_if user = guest` shortcut in `system-auth`.** Every service that includes `system-auth` would let the guest in without a password.
- **Autologin-style "log in without asking" through the normal service.** Same service as real users; one wrong condition opens real accounts.

## 5. Limits and behaviour

**One guest at a time.** A second guest would need a second account (fixed accounts) and the uid sweep would no longer be exact. While a guest session runs, the Guest entry is marked as logged in, like any logged-in user (reclogin decision 8).

**Guest next to logged-in users: allowed.** A person can switch user and hand the machine to a guest, then come back. The guest's session gets the devices while it is in front, and loses them when switched away; other users' sessions are locked by their own desktops. The guest cannot read their homes (0700, see R1) or their processes' memory.

**Guest with no desktop installed:** no Guest entry. On a system whose last desktop was removed there is a text login and no reclogin (`docs/reclogin.md`, note of 2026-11-15). reclogin also hides Guest when no valid session file exists.

**What the guest sees on first start:** the plain default desktop of the session chosen, from `/etc/skel` and the guest defaults tree, as a brand-new user would. No account wizard, no wallet set-up. The language is the system's (reclogin decision 12).

**The guest defaults tree** `/usr/share/reclogin/guest-home/` is data, not code. reclogin copies it over the fresh home and knows nothing of what is in it. Each desktop group recipe that wants guest defaults drops its own files there, keeping "sessions are data": for example the plasma recipe turns off screen locking and KWallet for the guest (`.config/kscreenlockerrc`, `.config/kwalletrc`), the lxqt recipe does the same for its locker. reclogin's own file in it is the notice below.

**What the guest is told:**

- On the login screen: **Guest** — "Nothing is kept after logout". (Already in the draft.)
- When the session starts: one desktop notification, sent by a small reclogin helper over the session bus's standard `org.freedesktop.Notifications` interface, started from an autostart entry in the guest defaults tree: **"You are using a guest session. Everything here is deleted when you log out. Save what you want to keep on a USB drive."** Plasma and LXQt both show these; reclogin draws nothing inside the session.
- At logout: nothing extra. The desktop's own logout is the moment.

**Screen locking: the guest session does not lock.** It has no password to unlock with. The desktops' guest defaults turn off automatic locking and locking on suspend. If a guest locks anyway (a key combination), the lock screen cannot be opened. For that case: choosing the logged-in Guest in reclogin, after "Switch user" from that lock screen or from another session, offers **"Return to the guest session"** and **"End the guest session"**. Ending it runs the logout and cleanup, then a fresh guest can start. This loses the guest's unsaved work, which the guest was told is never kept. It is the only time reclogin ends a session, and only the guest's.

**Automatic login may target the guest.** It is the public-machine case (a library computer, a shop demo). The setting names `guest` the same way it names a person. After the guest logs out the login screen appears; it does not start a new guest by itself (smaller point in `docs/reclogin.md`). If the guest entry is switched off, automatic login to it is refused and the login screen appears.

Rejected:

- **Several guests at once.** Needs a pool of fixed accounts and per-account sweeps for a case nobody asked for.
- **Guest only when nobody else is logged in.** Handing the machine to a visitor without logging out is the main use.
- **Locking the guest session with a throwaway password shown on screen.** Anyone who saw the screen can unlock it; it only adds a step.
- **reclogin writing Plasma or LXQt settings itself.** Breaks "sessions are data"; each desktop's recipe owns its own files.

## 6. Who creates what

| Who | What |
| --- | --- |
| **reclogin's installed tree** (wherever reclogin is installed from: image or the login-screen group recipe; that choice is reclogin's own, not this document's) | the `reclogin` binary with the guest code inside; `usr/lib/sysusers.d/reclogin.conf` with `g guest 60600` and `u guest 60600:60600 'Guest' /run/reclogin/guest /bin/bash`; `/etc/pam.d/reclogin-guest`; `/etc/security/limits.d/50-reclogin-guest.conf`; `/etc/ssh/sshd_config.d/50-reclogin-guest.conf`; `/usr/share/polkit-1/rules.d/50-reclogin-guest.rules`; the notice helper and its autostart entry in `/usr/share/reclogin/guest-home/` |
| **Recipe or the image builder** | turns the sysusers lines into the account, the same way for both (`tools/recipe/src/accounts.rs`): password `!*`, no home directory created (the home is made at each login) |
| **Desktop group recipes** (plasma, lxqt, …) | their guest defaults in `/usr/share/reclogin/guest-home/` (no locking, no wallet) |
| **The installer** (`recsetup`, setup program) | nothing new for the account; it already installs the default set before it makes the people, so the guest exists first. It writes the guest setting only when the person turns the guest off, and may write automatic login to `guest` |
| **reclogin at boot** | runs the cleanup before showing the screen |
| **reclogin at guest login** | cleanup; checks the account; makes and mounts the home; copies the defaults; PAM `reclogin-guest`; starts the session; checks logind/elogind registered it |
| **reclogin at guest logout** | closes PAM; `TerminateUser`; uid sweep; unmounts; cleanup |

**The administrator setting.** One line in reclogin's own settings file, the same file that holds automatic login (reclogin decision 2):

```
guest = on      # or off
```

The default is **on**, as the owner chose a guest session. With `off` the entry is not drawn, the `reclogin-guest` service is never opened and automatic login to the guest is refused. The account stays; it cannot be used without reclogin.

Recommended place: `/etc/reclogin.conf`, the same as automatic login. It is system configuration. On A/B a change to it goes into the other root and applies after the next boot, like every other system change. If the owner wants a switch that applies at once, the file moves to `/var/lib/reclogin/` (shared by both roots). That is a question for the owner (below), and it should be answered once for both settings.

Rejected: **removing the account to switch guest off.** On A/B that is a system change to `/etc/passwd`, and switching back would need the account again. An account that only reclogin can use costs nothing while it is off.

## 7. What it does not do, and the risks that remain

It does not:

- keep anything of the guest's: files, settings, browser history, downloads, software installed "for me", running programs;
- give the guest admin rights, system-wide installs, other people's files, new network connections, or new printers;
- run more than one guest at a time;
- let a guest in through the console, ssh or `su`;
- make a new account per login, or touch `/etc` at run time;
- protect files the guest copies to a USB drive or sends over the network: those are the guest's to take;
- hide that a guest session happened. The system log records it.

Risks that remain:

- **R1. AcornOS homes may be readable by everyone.** busybox `adduser -D` (used by `system-setup.sh`) makes homes 0755 as far as I know; Fedora's `useradd` makes them 0700. If so, the guest (and every other user) can read people's homes on AcornOS today. The setup program must make every home 0700 before the guest ships. This is a fix outside this document, to confirm on an AcornOS system.
- **R2. Swap.** tmpfs pages can be written to swap. If the machine swaps to an unencrypted disk partition or file, pieces of a guest's files can stay on the disk after power loss until overwritten. Memory-only swap (zram) or encrypted swap avoids it. What the systems use for swap is not decided here.
- **R3. Logs and crash dumps.** On LevitateOS journald keeps a per-user journal for uid 60600, and systemd-coredump may store a crashed guest program's memory. Both stay after logout, readable by root. AcornOS's syslog records the logins. "Nothing is kept" is about the guest's files and settings, not the system's own records.
- **R4. Shared memory filesystems.** `/tmp` (if tmpfs), `/dev/shm` and `/run/user/60600` are memory the guest can fill outside its home's cap until logout. The home cap and `nproc` limit do not cover them.
- **R5. Things other services keep on the guest's behalf.** Print jobs already sent finish and CUPS keeps their history (job names). A Bluetooth device paired by the guest stays paired system-wide (bluez keeps pairings in `/var/lib/bluetooth`; polkit does not guard pairing). Removable-drive mounts are cleaned; data written to the drive is not. The cleanup list in point 3 is the place to add any such service that turns up.
- **R6. A determined local attacker with the machine.** A guest at the keyboard can still boot something else from USB if firmware allows it. Firmware passwords and disk encryption are outside reclogin.
- **R7. Untried on both systems.** That `TerminateUser`, the `-session` PAM lines, `DenyUsers` in `sshd_config.d` and the uaccess device rules behave the same under elogind as under logind is read from documentation and package contents, not seen on a running system. The first implementation must show each on both OSes.
- **R8. Polkit coverage is a deny list.** An action not on the list that asks for an administrator would still show an admin prompt in the guest session (see point 1).

## Questions only the owner can answer

1. **Where reclogin's settings file lives** (`guest =` and automatic login): `/etc/reclogin.conf`, a system change that applies after the next boot on A/B (recommended), or `/var/lib/reclogin/`, which applies at once and is shared by both roots.
2. **Guest default on or off** after installation. Recommended: on, and the installer asks nothing.
3. **May a guest join a new WiFi network?** Recommended: no, existing connections only, because NetworkManager stores a new connection on disk where it outlives the guest. Allowing it needs NetworkManager's in-memory connections, which the desktop network applets do not create.
4. **May a guest mount USB drives?** Recommended: yes (mount, unmount, eject only).
5. **Home size cap:** a quarter of memory, at most 4 GiB (recommended), or another figure.
6. **uid 60600**, or another number in 60578–61183.
7. **Automatic login to the guest**: allowed for public machines (recommended), or forbidden.

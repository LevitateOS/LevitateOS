"""system-setup, recab, the guide, and the install screen that holds the hub and the guide."""
import re
from kit import *
from pages_install import PAGES as _INSTALL, hub, STEP_KEYS, CONFIRM_KEYS

PAGES = []


def page(app, name, note, p):
    PAGES.append({"app": app, "name": name, "note": note, "page": p})
    return p


LIST_KEYS = [("↑/↓", "select"), ("PgUp/PgDn", "page"), ("Enter", "choose"), ("Bksp", "erase"), ("Esc", "cancel")]
TEXT_KEYS = [("Enter", "accept"), ("Bksp", "erase"), ("Esc", "cancel")]

# ---- the world, from the app's own land bitmap
import pathlib

LAND_TS = pathlib.Path(__file__).resolve().parents[2] / "tui/apps/install/system-setup/src/domain/land.ts"
LAND_ROWS = re.findall(r'^\t"([0-9a-f]+)",$', open(LAND_TS).read(), re.M)
NORTH, SOUTH = 78, -56


def is_land(lat, lon):
    y = int((NORTH - lat) / (NORTH - SOUTH) * len(LAND_ROWS))
    x = int(((lon + 180) % 360) / 360 * len(LAND_ROWS[0]) * 4)
    if not 0 <= y < len(LAND_ROWS):
        return False
    return bool(int(LAND_ROWS[y][x >> 2], 16) & (8 >> (x & 3)))


def world(p, x, y, cols, rows, pin, lit=()):
    bits_of = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]]
    cell = lambda lat, lon: (min(rows - 1, int((NORTH - lat) / (NORTH - SOUTH) * rows)), min(cols - 1, int((lon + 180) / 360 * cols)))
    marked = {cell(*place) for place in lit}
    pinned = cell(*pin) if pin else None
    for r in range(rows):
        for c in range(cols):
            bits = 0
            for dy in range(4):
                for dx in range(2):
                    lon = -180 + (c * 2 + dx + 0.5) / (cols * 2) * 360
                    lat = NORTH - (r * 4 + dy + 0.5) / (rows * 4) * (NORTH - SOUTH)
                    if (c * 2 + dx + r * 4 + dy) % 2 == 0 and is_land(lat, lon):
                        bits |= bits_of[dy][dx]
            if (r, c) == pinned:
                p.put(y + r, x + c, "●", p.highlight, INK, bold=True)
            elif bits:
                p.put(y + r, x + c, chr(0x2800 + bits), p.accent if (r, c) in marked else LICHEN, INK, bold=(r, c) in marked)


# ====================================================================== system-setup
SS = ["Overview", "Keyboard", "Time zone", "Language", "Hostname", "Root password", "User", "Apply", "Done"]
SS_VALUES = ["", "de", "Amsterdam", "en_US", "studio", "set", "ann", "6 steps", ""]


def ss(selected, done, keys=STEP_KEYS, right="/mnt", distro="levitate", states=None):
    p = Page(distro=distro)
    p.band("Configuration", "Overview" if selected == 0 else "Done" if selected == 8 else f"step {selected} of 7")
    p.steps(7, max(0, min(7, done - 1)), selected - 1 if 0 < selected < 8 else None)
    steps = [(label, "done" if i < done else "ready" if i == done else "todo", SS_VALUES[i] if i < done else "") for i, label in enumerate(SS)]
    for i, state in (states or {}).items():
        steps[i] = (steps[i][0], state, steps[i][2])
    p.path(steps, selected)
    p.keys(keys, right)
    return p


EUROPE = [("Amsterdam", "02:20", "+01:00", 52.4, 4.9), ("Athens", "03:20", "+02:00", 38.0, 23.7), ("Berlin", "02:20", "+01:00", 52.5, 13.4),
          ("Brussels", "02:20", "+01:00", 50.8, 4.3), ("Dublin", "01:20", "+00:00", 53.3, -6.3), ("Helsinki", "03:20", "+02:00", 60.2, 25.0),
          ("Lisbon", "01:20", "+00:00", 38.7, -9.1), ("London", "01:20", "+00:00", 51.5, -0.1), ("Madrid", "02:20", "+01:00", 40.4, -3.7),
          ("Oslo", "02:20", "+01:00", 59.9, 10.7), ("Paris", "02:20", "+01:00", 48.9, 2.3), ("Prague", "02:20", "+01:00", 50.1, 14.4)]


def zone_page(distro="levitate"):
    p = ss(2, 2, LIST_KEYS, distro=distro)
    p.heading(2, "Where is this machine?", "The clock of the new system.")
    list_w = 38
    top = p.y
    p.field("Search", "eu", "64 of 598", width=list_w)
    p.gap()
    p.rows_list([("Europe/", city, clock, offset) for city, clock, offset, _, _ in EUROPE], 0, list_w, match="eu")
    p.put(p.y, p.x + 1, "52 more below", GUTTER)
    card_x, card_w = p.x + list_w + 2, p.w - list_w - 2
    for r in range(top, top + 16):
        p.fill(r, card_x, card_w, INK)
    p.put(top + 1, card_x + 2, "Amsterdam", PAPER, INK, bold=True)
    p.put(top + 1, card_x + card_w - 7, " CET ", NIGHT, p.accent, bold=True)
    p.put(top + 2, card_x + 2, "Netherlands, Europe", LICHEN, INK)
    world(p, card_x + 2, top + 4, card_w - 4, 8, (52.4, 4.9), [(lat, lon) for *_, lat, lon in EUROPE])
    p.y = top + 13
    for label, value, bold in [("Local time", "02:20", True), ("From UTC", "+01:00", False)]:
        p.put(p.y, card_x + 2, label, LICHEN, INK)
        p.put(p.y, card_x + card_w - 2 - len(value), value, PAPER, INK, bold=bold)
        p.y += 1
    return p


def system_setup_pages():
    p = page("system-setup", "Overview", "The opening page lists the questions; each will show its answer here as it is given.", ss(0, 1))
    p.heading("diamond", "Configure the new system", "Six questions, then the commands to review. Nothing changes before you say so.")
    p.section("System")
    p.facts([("System", "LevitateOS"), ("Mounted at", "/mnt"), ("Administrators", "members of wheel, with sudo")])
    p.gap().section("What you choose")
    p.steplist([("", [("ready", "Keyboard layout"), ("todo", "Time zone"), ("todo", "Language"), ("todo", "Hostname"), ("todo", "Root password"), ("todo", "A user for yourself")])])
    p.gap().hint([("Enter",), " starts with the keyboard."])

    p = page("system-setup", "1 Keyboard", "A list question: a search field, the list, the selection bar.", ss(1, 1, LIST_KEYS))
    p.heading(1, "Which keyboard do you type on?", "Loaded on this console at once, so your passwords match after the reboot.")
    p.field("Search", "", "22 of 22", width=40)
    p.gap()
    p.rows_list([("", name, "", "") for name in ["pl", "pt-latin1", "ru", "se-latin1", "sg", "uk", "us", "us-acentos"]], 6, 40)
    p.put(p.y, p.x + 1, "14 above", GUTTER)

    page("system-setup", "2 Time zone", "The list with each zone's clock, and the selected zone on the map.", zone_page())

    p = page("system-setup", "3 Language", "The same list, narrowed by typing; the matched letters are picked out.", ss(3, 3, LIST_KEYS))
    p.heading(3, "Which language?", "The system's language and formats.")
    p.field("Search", "en", "2 of 16", width=40)
    p.gap()
    p.rows_list([("", "en_GB.UTF-8", "", ""), ("", "en_US.UTF-8", "", "")], 1, 40, match="en")

    p = page("system-setup", "4 Hostname, refused", "A typed answer: one field. recsetup's own refusal and fix appear under it, in the danger panel.", ss(4, 4, TEXT_KEYS))
    p.heading(4, "What is this machine called?", "Its name on the prompt and on the network.")
    p.field("Hostname", "studio_pc", width=40)
    p.gap()
    p.panel("danger", "'studio_pc' is not a valid hostname", ["What to do: use letters, digits and hyphens (not first or last), at most 64 characters, such as my-pc."])

    p = page("system-setup", "5 Root password", "A password is typed twice and shown as dots; the first entry stays visible as done while the second is typed.", ss(5, 5, TEXT_KEYS))
    p.heading(5, "A password for root?", "Leave it empty and root gets none: only your own user administers, with sudo.")
    p.field("Password ", "••••••••••", width=40, cursor=False)
    p.put(p.y - 1, p.x + 42, "✓", LEAF)
    p.gap()
    p.field("Once more", "••••", width=40)
    p.gap().hint(["It is handed to recsetup directly: never part of a command line or a file."])

    p = page("system-setup", "6 User", "The same field for the user's name; the password follows on the same page.", ss(6, 6, TEXT_KEYS))
    p.heading(6, "Who uses this machine?", "Your own account, an administrator (sudo). Leave the name empty to create none.")
    p.field("User name", "ann", width=40)
    p.gap().hint([("Enter",), " accepts the name and asks for its password."])

    p = page("system-setup", "7 Apply, reviewing", "The answers and the exact commands, before anything runs.", ss(7, 7))
    p.heading(7, "Review and apply", "recsetup runs these in order and stops at the first that fails.")
    top = p.y
    p.section("Your answers")
    half = p.w // 2 - 2
    p.facts([("Keyboard", "de"), ("Time zone", "Europe/Amsterdam"), ("Language", "en_US.UTF-8")])
    p.y = top + 1
    p.facts([("Hostname", "studio"), ("Root password", "set"), ("User", "ann, admin")], x=p.x + half + 4)
    p.gap().section("What will run", "6 steps")
    p.steplist([("", [("todo", "Set the time zone to Europe/Amsterdam"), ("todo", "Set the system locale to en_US.UTF-8"), ("todo", "Set the console keymap to de"),
                      ("todo", "Set the hostname and hosts mapping to studio"), ("todo", "Create user ann"), ("todo", "Set the password of root, ann")])])
    p.gap().hint([("Enter",), " applies them; you confirm first.   ", ("space",), " shows the commands."])

    p = page("system-setup", "7 Apply, asking", "The yes-or-no panel, at the top as everywhere.", ss(7, 7, CONFIRM_KEYS))
    p.heading(7, "Review and apply", "recsetup runs these in order and stops at the first that fails.")
    p.panel("warning", "Apply these 6 steps to /mnt?", ["Nothing has changed yet."], keys=[("y", "apply"), ("n", "not yet")])
    p.section("What will run", "6 steps")
    p.steplist([("", [("todo", "Set the time zone to Europe/Amsterdam"), ("todo", "Set the system locale to en_US.UTF-8"), ("todo", "Set the console keymap to de"),
                      ("todo", "Set the hostname and hosts mapping to studio"), ("todo", "Create user ann"), ("todo", "Set the password of root, ann")])])

    p = page("system-setup", "7 Apply, failed", "A failure names the step, shows what it printed and what ran; the steps keep their marks.", ss(7, 7, right="stopped", states={7: "failed"}))
    p.heading("bang", "The configuration stopped", "recsetup stopped at the first step that failed.", SALMON)
    p.panel("danger", "'Create user ann' failed with exit status 9", ["useradd: cannot lock /etc/passwd; try again later.", "What to do: fix what the step's output says, then apply again."])
    p.steplist([("", [("done", "Set the time zone to Europe/Amsterdam"), ("done", "Set the system locale to en_US.UTF-8"), ("done", "Set the console keymap to de"),
                      ("done", "Set the hostname and hosts mapping to studio"), ("failed", "Create user ann"), ("todo", "Set the password of root, ann")])])
    p.gap().hint([("Enter",), " applies again.   A shell is one key away in the install steps."])

    p = page("system-setup", "Done", "The shared ending.", ss(8, 9))
    p.heading("check", "System configured", "It knows its keyboard, clock, language and name, and who may log in.", LEAF)
    p.panel("success", "Next: boot", ["Install the boot loader and the boot entries for the disk's layout."])
    p.section("What was set")
    top = p.y
    p.facts([("Keyboard", "de"), ("Time zone", "Europe/Amsterdam"), ("Language", "en_US.UTF-8")])
    p.y = top
    p.facts([("Hostname", "studio"), ("Root password", "set"), ("User", "ann, admin")], x=p.x + p.w // 2 + 2)
    p.gap().hint([("Enter",), " goes back to the install steps."])

    page("system-setup", "2 Time zone, AcornOS", "The same page on AcornOS: its band and its accent, nothing else differs.", zone_page("acorn"))


# ====================================================================== recab
def recab(selected, running="A", trial=None, keys=None, right=None, history=""):
    p = Page()
    place = ["Overview", "Try slot B", "Commit", "Roll back", "Write a system", "Reboot", "History"][selected]
    p.band("Updates", place)
    other = "B" if running == "A" else "A"
    labels = [f"Slot {running} · running", f"Slot {other} · next boot, once" if trial else f"Slot {other} · previous system"]
    p.steps(2, labels=labels if running == "A" else labels[::-1], lit=[running == "A", running == "B"])
    if trial:
        seg = (p.cols - 2 - 1) // 2
        start = 1 + (1 if running == "A" else 0) * (seg + 1)
        p.fill(2, start, seg, p.highlight)
        p.put(2, start + 2, labels[1], NIGHT, bold=True)
    p.menu([
        (None, [("Overview", "none", "o")]),
        ("Changes", [(f"Try slot {other}", "ready", "t"), (f"Commit slot {running}", "todo", "c"), (f"Roll back to {other}", "ready", "x"),
                     (f"Write to slot {other}", "ready", "s"), ("Reboot", "ready", "R")]),
        ("Session", [("History" + history, "none", "H")]),
    ], selected)
    p.keys(keys or [("j/k", "select"), ("Enter", "run it"), ("r", "read again"), ("q", "quit")], right or f"slot {running}")
    return p


def slot_cards(p, running, tags_a, tags_b):
    p.cards([("Slot A", "running" if running == "A" else "", tags_a, running == "A"), ("Slot B", "running" if running == "B" else "", tags_b, running == "B")], height=5)


def recab_pages():
    p = page("recab", "Overview", "The installed system's updates. The line under the band is the two slots; the cards say what each holds.", recab(0))
    p.heading("slots", "A/B slots", "The system runs from one slot; an update goes into the other and is tried once.")
    p.panel("success", "Running slot A, committed", ["Slot A is known-good and boots by default. Slot B still holds the previous system."])
    slot_cards(p, "A", ["known-good", "boots by default", "next boot"], ["not running", "roll back target", "last trial: failed"])
    p.section("Boot order")
    p.facts([("Running now", "slot A"), ("Next boot", "slot A"), ("Roll back to", "slot B")])

    p = page("recab", "Overview, a trial pending", "A pending trial lights the other slot in the second accent, on the slot line and in the panel.", recab(0, trial=True))
    p.heading("slots", "A/B slots", "The system runs from one slot; an update goes into the other and is tried once.")
    p.panel("info", "Slot B boots once on the next reboot", ["If it starts healthy, recab commits it and it becomes the default. If it fails, the machine reboots into slot A."])
    slot_cards(p, "A", ["known-good", "boots by default"], ["not running", "next boot, as a trial", "last trial: failed"])
    p.section("Boot order")
    p.facts([("Running now", "slot A"), ("Next boot", "slot B, once, as a trial"), ("If it fails", "back to slot A")])

    p = page("recab", "A change, before", "Every change shows what it would alter, now against after, and its command.", recab(1))
    p.heading("slots", "Try slot B on the next boot", "Boot it once. If it starts healthy it is kept; if not, the machine comes back to slot A.")
    p.section("What changes")
    p.table(["", "Now", "After"], [["Next boot", "A", "B, once"], ["Trial pending", "none", "B"], ["Default", "A", "A"], ["If the trial fails", "", "back to A"]])
    p.gap().section("Command")
    p.commands(["recab set-next B"])
    p.gap().hint([("Enter",), " runs it; you confirm first."])

    p = page("recab", "A change, asking", "The same yes-or-no panel as the install apps.", recab(1, keys=CONFIRM_KEYS))
    p.heading("slots", "Try slot B on the next boot", "Boot it once. If it starts healthy it is kept; if not, the machine comes back to slot A.")
    p.panel("warning", "Run recab set-next B?", ["Nothing changes until the next reboot."], keys=[("y", "run it"), ("n", "cancel")])
    p.section("What changes")
    p.table(["", "Now", "After"], [["Next boot", "A", "B, once"], ["Trial pending", "none", "B"], ["Default", "A", "A"]])

    p = page("recab", "A change, done", "After it ran: the outcome, then before against after.", recab(1, trial=True, history=" · 1"))
    p.heading("check", "Slot B boots once on the next reboot", "Reboot to start the trial.", LEAF)
    p.panel("success", "recab set-next B", ["Took 0.1 s."])
    p.section("Before and after")
    p.table(["", "Before", "After"], [["Running", "A, committed", "A, committed"], ["Next boot", "A", "B, once"], ["Default", "A", "A"], ["Known-good", "A", "A"]],
            colours=[None, p.accent, None, None])
    p.gap().hint([("R",), " goes to Reboot."])

    p = page("recab", "A change recab would refuse", "A refusal is known before anything runs: the danger panel carries recab's own reason.", recab(3))
    p.heading("slots", "Roll back", "Return to the previous system: the other slot becomes the default and known-good.")
    p.panel("danger", "recab would refuse this now", ["Slot A is committed and no other slot has been known-good since it was written: there is nothing to roll back to."])
    p.section("Command")
    p.commands(["recab rollback"])

    p = page("recab", "Write a system", "A change that needs answers asks for them one field at a time; the answered ones stay above, ticked.",
             recab(4, keys=[("Enter", "accept"), ("Tab", "complete"), ("Bksp", "erase"), ("Esc", "cancel")]))
    p.heading("slots", "Write a new system to slot B", "Format slot B and write a system into it, carrying accounts, hostname, network and services over.")
    p.field("System image", "/run/media/levitateos/live/filesystem.erofs", width=p.w, cursor=False)
    p.put(p.y - 1, p.x + p.w - 3, "✓", LEAF, RAISED)
    p.gap()
    p.field("Kernel      ", "/run/media/levitateos/boot/vmli", width=p.w)
    p.gap()
    p.field("Initramfs   ", "", width=p.w, cursor=False)
    p.gap().section("What changes")
    p.table(["", "Now", "After"], [["Slot B", "previous system", "new system"], ["Roll back to", "B", "none"], ["Next boot", "A", "A"]])

    p = page("recab", "Write a system, the last word", "Formatting a slot asks for the slot's name, as disk-plan asks for DESTROY: destructive steps always need a typed word.",
             recab(4, keys=[("Enter", "write it, once unlocked"), ("Bksp", "erase"), ("Esc", "cancel")]))
    p.heading("slots", "Write a new system to slot B", "Format slot B and write a system into it, carrying accounts, hostname, network and services over.")
    p.panel("danger", "This formats slot B", ["The previous system in it is gone, and with it the roll back target."])
    p.section("Type B to unlock")
    p.field("›", "", "locked", width=40)
    p.gap().section("Command")
    p.commands(["recab stage --rootfs /run/media/levitateos/live/filesystem.erofs --kernel /run/media/levitateos/boot/vmlinuz"])

    p = page("recab", "History", "What ran in this session, newest first.", recab(6, history=" · 2"))
    p.heading("slots", "History", "What ran in this session. recab keeps its own log in /var/lib/recab/audit.log.")
    p.table(["When", "Change", "Result", "Took"], [["14:02:11", "recab set-next B", "done", "0.1 s"], ["14:01:48", "recab rollback", "refused", "0.0 s"]],
            colours=[None, SALMON])

    p = page("recab", "No A/B state", "When recab cannot read the system, the page says so in recab's words and offers one key.", recab(0, right="no status"))
    p.heading("bang", "No A/B layout here", "recab could not read the slots.", SALMON)
    p.panel("danger", "No slot entries under /boot/loader/entries", ["This system was installed with one mutable root, or the ESP is not mounted at /boot."])
    p.hint([("r",), " reads it again."])


# ====================================================================== the guide
GUIDE = ["Getting Started", "Installation", "Disk Preparation", "Base System", "Configuration", "Boot & Finish", "Post-Installation", "Troubleshooting"]


def guide(selected, cols=100, rows=28, distro="levitate", brand=True, rail=30):
    p = Page(cols, rows, distro, rail=rail)
    p.band("Guide", GUIDE[selected], brand=brand)
    p.steps(len(GUIDE), selected, selected)
    p.menu([("Getting started", [(name, "done" if i < selected else "none" if i != selected else "ready", "") for i, name in enumerate(GUIDE)]),
            (None, [("Package manager", "none", "▸"), ("Helpers", "none", "▸"), ("Architecture", "none", "▸"), ("Rec tooling", "none", "▸")])], selected)
    p.keys([("j/k", "scroll"), ("Tab", "pages"), ("Enter", "copy command"), ("q", "quit")], f"page {selected + 1} of 34")
    return p


def guide_disk(p):
    p.heading(3, "Disk Preparation", "Installation steps 1-5: identify the disk, plan the partitions, format and mount.")
    p.panel("info", "These steps in one tool", ["disk-plan chooses the disk and the layout, shows every command, and applies them after you type DESTROY."])
    p.runs(p.y, p.x, [("2 ", p.accent, True), ("Identify Target Disk", PAPER, True)])
    p.y += 1
    p.text("List the disks and find the one to install on. Everything on it will be erased.", MOSS)
    p.gap()
    p.commands(["lsblk -o NAME,SIZE,MODEL,TRAN"], label="bash")
    p.gap()
    p.runs(p.y, p.x, [("3 ", p.accent, True), ("Plan the Partitions", PAPER, True)])
    p.y += 1
    p.table(["Partition", "Size", "Type", "Mount"], [["EFI", "1 GiB", "vfat", "/boot"], ["root A", "20 GiB", "ext4", "/"], ["root B", "20 GiB", "ext4", "none"], ["state", "rest", "ext4", "/var"]])
    return p


def guide_pages():
    page("guide", "A guide page", "The guide beside the install steps. Its line under the band is the chapters; notes are panels; commands are blocks.", guide_disk(guide(2)))

    p = page("guide", "A command selected", "A selected command is the selection bar inside its block; one key copies it.", guide(3))
    p.heading(4, "Base System", "Installation steps 6-9: verify the media, extract the system, generate the fstab.")
    p.runs(p.y, p.x, [("7 ", p.accent, True), ("Extract Base System", PAPER, True)])
    p.y += 1
    p.text("Use recstrap to extract the system image to your mounted root:", MOSS)
    p.gap()
    p.fill(p.y, p.x, p.w, p.accent)
    p.put(p.y, p.x + 2, "$ recstrap /mnt", NIGHT, p.accent, bold=True)
    p.put(p.y, p.x + p.w - 14, " Enter  copy ", NIGHT, p.accent)
    p.y += 2
    p.text("This extracts the complete base system: systemd, the userland, kernel modules, NetworkManager, sudo and the recipe package manager.", MOSS)
    p.gap()
    p.runs(p.y, p.x, [("8 ", p.accent, True), ("Generate fstab", PAPER, True)])
    p.y += 1
    p.commands(["recfstab --write /mnt"], label="bash")


# ====================================================================== the install screen
def install_screen(distro="levitate"):
    """The hub and the guide side by side, as the live ISO opens: two panes of one
    screen, one band, the pane without the keyboard quieter."""
    cols, rows = 160, 30
    half = cols // 2
    left = hub(2, 2, half, rows - 1, distro, right="")
    left.heading(3, "Copy the system onto the disk", "Check the target, copy the system, write its fstab.")
    left.panel("info", "Next: base-system", ["It shows a progress bar while it copies and asks before it replaces anything in the fstab."])
    left.section("Starts")
    left.commands(["base-system --then hub"])
    left.gap().hint([("Enter",), " starts it.   The guide beside this has it in full."])
    right = guide(3, half, rows - 1, distro, brand=False, rail=28)
    right.heading(4, "Base System", "Installation steps 6-9: verify the media, extract the system, generate the fstab.")
    right.runs(right.y, right.x, [("6 ", right.accent, True), ("Verify Installation Media", PAPER, True)])
    right.y += 1
    right.text("The live media is already mounted. Verify the system image it installs is there:", MOSS)
    right.gap()
    right.commands(["ls -la /run/live-rootfs.erofs"], label="bash")
    right.gap()
    right.runs(right.y, right.x, [("7 ", right.accent, True), ("Extract Base System", PAPER, True)])
    right.y += 1
    right.text("Use recstrap to extract the system image to your mounted root:", MOSS)
    right.gap()
    right.commands(["recstrap /mnt"], label="bash")
    right.gap()
    right.runs(right.y, right.x, [("8 ", right.accent, True), ("Generate fstab", PAPER, True)])
    right.y += 1
    right.commands(["recfstab --write /mnt"], label="bash")
    right.dim(0, half, 0.35)
    whole = Page(cols, rows, distro, rail=0)
    for r in range(rows - 1):
        whole.grid[r] = left.grid[r] + right.grid[r]
    whole.fill(rows - 1, 0, cols, INK)
    whole.put(rows - 1, 2, "The install steps have the keyboard", GUTTER, INK)
    col = cols - 40
    for key, label in [("Ctrl-]", "other pane"), ("Ctrl-q", "quit")]:
        col = whole.put(rows - 1, col, f" {key} ", PAPER, RULE, bold=True)
        col = whole.put(rows - 1, col + 1, label, LICHEN, INK) + 3
    return whole


def install_screen_pages():
    page("install screen", "The install screen", "What the live ISO opens: the install steps and the guide as two panes of one screen. No boxes; the pane without the keyboard is quieter; one line of keys for the screen itself.", install_screen())
    page("install screen", "The install screen, AcornOS", "The same screen on AcornOS.", install_screen("acorn"))


install_screen_pages()
PAGES.extend(_INSTALL)
system_setup_pages()
recab_pages()
guide_pages()

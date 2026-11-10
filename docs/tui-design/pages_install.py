"""The install: the hub, disk-plan and base-system.

Third pass: balance. Each page keeps the one picture of what it does, and loses whatever
competed with it. A page has one thing to look at and at most one thing to press, and the
thing to press is always on the same row."""
from kit import *

PAGES = []


def page(app, name, note, p):
    PAGES.append({"app": app, "name": name, "note": note, "page": p})
    return p


STEP_KEYS = [("j/k", "step"), ("Enter", "go on"), ("q", "back")]
CONFIRM_KEYS = [("y", "yes"), ("n", "no")]

# ====================================================================== install hub
HUB_STEPS = ["Preflight", "Disk", "Base system", "Configuration", "Boot", "Finish"]
HUB_VALUES = ["UEFI", "nvme0n1", "copied", "ann", "A/B", ""]


def hub(selected, done, cols=100, rows=28, distro="levitate", right="state in /run/install"):
    p = Deliberate(cols, rows, distro)
    p.band("Install", f"step {selected + 1} of 6")
    p.steps(6, done, selected)
    p.path([(label, "done" if i < done else "ready" if i == done else "todo", HUB_VALUES[i] if i < done else "")
            for i, label in enumerate(HUB_STEPS)], selected)
    p.keys([("j/k", "step"), ("Enter", "do it"), ("d", "mark done"), ("s", "shell"), ("q", "quit")], right)
    return p


def tile(p, x, w, tone, title, lines):
    """A check as a tile: its verdict, and what was found."""
    colour = TONES[tone]
    top = p.y
    p.card(x, top, w, 1 + len(lines), RAISED)
    for r in range(top, top + 1 + len(lines)):
        p.put(r, x, "█", colour)
    p.put(top, x + 2, f"{'✗' if tone == 'danger' else '✓'} {title}", colour, RAISED, bold=True)
    for i, (text, tone_) in enumerate(lines):
        p.put(top + 1 + i, x + 2, text, tone_, RAISED)


def from_to(p, source, target):
    """What goes where: two surfaces and an arrow."""
    half = (p.w - 5) // 2
    top = p.y
    for i, (title, lines) in enumerate([source, target]):
        x = p.x + i * (half + 5)
        p.card(x, top, half, 1 + len(lines) + 2)
        p.put(top + 1, x + 2, title, PAPER, INK, bold=True)
        for j, line in enumerate(lines):
            p.put(top + 2 + j, x + 2, line, LICHEN, INK)
    p.put(top + 2, p.x + half + 1, "──▶", p.accent, bold=True)
    p.y = top + len(source[1]) + 4


def launch(p):
    """What the hub shows for the step you are at: from what onto what, and the bar."""
    p.heading(3, "Copy the system onto the disk", "A few minutes of copying, then the mounts are written down.")
    from_to(p, ("The live image", ["/run/live-rootfs.erofs", "1.4 GiB"]), ("/mnt", ["/dev/nvme0n1p5, slot A", "30.0 GiB, empty"]))
    p.button("Start base-system")


def hub_pages():
    p = page("install-hub", "1 Preflight", "Two tiles and a bar. The rail already shows the road ahead, so the page does not repeat it.", hub(0, 0))
    p.heading(1, "Can this machine take the install?", "Two things must hold before anything is touched.")
    half = (p.w - 2) // 2
    tile(p, p.x, half, "success", "Booted in UEFI mode", [("systemd-boot needs it", LICHEN)])
    tile(p, p.x + half + 2, half, "success", "Install media mounted", [("/run/live-rootfs.erofs", LICHEN)])
    p.y += 3
    p.text("Nothing on any disk changes before you type DESTROY in the Disk step.", LICHEN)
    p.button("Continue to the disk")

    p = page("install-hub", "1 Preflight, a check failed", "The failed tile turns, and the line under the tiles says what to do. There is nothing to press.", hub(0, 0))
    p.heading("bang", "This boot cannot take the install", "One of the two things does not hold.", SALMON)
    half = (p.w - 2) // 2
    tile(p, p.x, half, "danger", "Not booted in UEFI mode", [("systemd-boot needs it", LICHEN)])
    tile(p, p.x + half + 2, half, "success", "Install media mounted", [("/run/live-rootfs.erofs", LICHEN)])
    p.y += 3
    p.text("Reboot, open the firmware's boot menu, and pick the UEFI entry for the install media.")

    p = page("install-hub", "3 A step ready to start", "The step you are at: what goes where, and one bar.", hub(2, 2))
    launch(p)

    p = page("install-hub", "2 A finished step", "A done step is the disk it left. Doing it again is one quiet line with its cost.", hub(1, 3))
    p.heading(2, "The disk is laid out", "Done at 14:02.")
    spans = p.diskbar([("", 4, MOSS), ("Windows", 300, GUTTER), ("A", 30, p.accent), ("B", 30, LEAF), ("state", 112, SAND)], 476)
    for (col, _), mount in zip(spans, ["/boot", "", "/mnt", "", "/var"]):
        p.put(p.y, col, mount, LICHEN)
    p.note("Enter opens disk-plan again. That drops every step after it.")

    p = page("install-hub", "5 A step that waits", "A step ahead shows what it waits for and what it will leave. Nothing to press yet.", hub(4, 2))
    p.heading(5, "Install the boot loader", "systemd-boot, and one boot entry for each slot.")
    p.card(p.x, p.y, p.w, 5)
    p.put(p.y + 1, p.x + 2, "/boot", PAPER, INK, bold=True)
    for i, (entry, note, colour) in enumerate([("LevitateOS (slot A)", "the default", p.accent), ("LevitateOS (slot B)", "empty until the first update", GUTTER)]):
        p.put(p.y + 2 + i, p.x + 2, "▸", colour, INK)
        p.put(p.y + 2 + i, p.x + 4, entry, PAPER, INK)
        p.put(p.y + 2 + i, p.x + 28, note, LICHEN, INK)
    p.note("Waits for the base system and the configuration.")

    p = page("install-hub", "6 Finish", "The machine you made, one reminder, one bar.", hub(5, 5))
    p.heading("check", "Ready to reboot", "The install is complete.", LEAF)
    p.card(p.x, p.y, p.w, 7)
    p.fill(p.y, p.x, p.w, p.accent)
    p.put(p.y, p.x + 2, "studio", NIGHT, p.accent, bold=True)
    p.put(p.y, p.x + p.w - 16, "LevitateOS 1.0", NIGHT, p.accent)
    top = p.y + 2
    p.y = top
    p.facts([("Logs in", "ann, an administrator"), ("Keyboard", "de"), ("Clock", "Europe/Amsterdam")], x=p.x + 2, bg=INK)
    p.y = top
    p.facts([("Disk", "nvme0n1, beside Windows"), ("Layout", "A/B, slot A"), ("Language", "en_US.UTF-8")], x=p.x + p.w // 2 + 3, bg=INK)
    p.y = top + 6
    p.text("Remove the install media when the machine restarts.", SAND)
    p.button("Unmount and reboot")


# ====================================================================== disk-plan
# One story through every page: a laptop with Windows on it, kept beside the new system.
DISK = "/dev/nvme0n1"
NOW = [("", 4, MOSS), ("Windows", 468, GUTTER), ("", 4, RULE)]
DP = ["Overview", "Disk", "System", "Disk use", "Partitions", "Plan", "Commands", "Checks", "Confirm", "Apply", "Done"]
DP_VALUES = ["", "nvme0n1", "A/B", "keep some", "5 edits", "4 change", "10", "passed", "", "", ""]


def after(p):
    return [("", 4, MOSS), ("Windows", 300, GUTTER), ("A", 30, p.accent), ("B", 30, LEAF), ("state", 108, SAND), ("", 4, RULE)]


def dp(selected, done, keys=STEP_KEYS, right=DISK, states=None):
    p = Deliberate()
    total = len(DP) - 1
    p.band("Disk", "Overview" if selected == 0 else f"step {selected} of {total}")
    p.steps(total, max(0, done - 1), selected - 1 if selected else None)
    steps = [(label, "done" if i < done else "ready" if i == done else "todo", DP_VALUES[i] if i < done else "") for i, label in enumerate(DP)]
    for i, state in (states or {}).items():
        steps[i] = (steps[i][0], state, steps[i][2])
    p.path(steps, selected)
    p.keys(keys, right)
    return p


RUN = [
    ("Resize", ["ntfsresize --size 300G /dev/nvme0n1p3", "sfdisk --no-reread -N 3 /dev/nvme0n1"]),
    ("Partition", ["sfdisk --append /dev/nvme0n1", "udevadm settle --timeout=10"]),
    ("Format", ["mkfs.ext4 -F -L ROOT_A /dev/nvme0n1p5", "mkfs.ext4 -F -L ROOT_B /dev/nvme0n1p6", "mkfs.ext4 -F -L STATE /dev/nvme0n1p7"]),
    ("Mount", ["mount /dev/nvme0n1p5 /mnt", "mount /dev/nvme0n1p1 /mnt/boot", "mount /dev/nvme0n1p7 /mnt/var"]),
]


def runsheet(p, upto=None, failed=None):
    """The commands in the order they run, a phase named where it starts. `upto` are
    done and the next is running; `failed` is the number that failed."""
    n = 0
    for phase, lines in RUN:
        for i, line in enumerate(lines):
            n += 1
            state = "todo" if upto is None else "failed" if n == failed else "done" if n <= upto else "running" if n == upto + 1 and failed is None else "todo"
            quiet = state == "todo" and upto is not None
            here = state in ("running", "failed")
            if here:
                p.fill(p.y, p.x + 11, p.w - 11, RAISED)
            p.put(p.y, p.x, phase if i == 0 else "", GUTTER if quiet else MOSS)
            mark, colour = MARKS[state]
            p.put(p.y, p.x + 12, mark, colour or p.accent, RAISED if here else None, bold=here)
            p.put(p.y, p.x + 15, line, LICHEN if quiet else PAPER, RAISED if here else None, bold=here)
            p.y += 1


def disk_plan_pages():
    p = page("disk-plan", "Overview", "One picture: the line where writing starts. The three stretches are words on it, not boxes.", dp(0, 0, right="no disk yet"))
    p.heading(None, "Prepare the install disk", "Every command is shown before anything is written.")
    p.gap()
    safe = round(p.w * 8 / 10)
    third = safe // 2
    for i, (title, steps_) in enumerate([("Choose", "the disk and the layout"), ("Review", "the plan and its commands")]):
        p.put(p.y, p.x + i * third, title, PAPER, bold=True)
        p.put(p.y + 1, p.x + i * third, steps_, LICHEN)
    p.put(p.y, p.x + safe + 2, "Apply", PAPER, bold=True)
    p.put(p.y + 1, p.x + safe + 2, "it is written", LICHEN)
    p.y += 3
    p.put(p.y, p.x, "█" * safe, LEAF)
    p.put(p.y, p.x + safe, "█", PAPER)
    p.put(p.y, p.x + safe + 1, "█" * (p.w - safe - 1), SALMON)
    p.put(p.y + 2, p.x, "Nothing is written before you type DESTROY.", MOSS)
    p.button("Choose a disk")

    p = page("disk-plan", "1 Disk", "Each disk is its bar. A name, a size, one line under it; nothing else.", dp(1, 1, keys=[("↑/↓", "select"), ("Enter", "choose"), ("R", "read again"), ("q", "back")], right="3 disks"))
    p.heading(1, "Which disk?", "Nothing is written before the Confirm step.")
    disks = [("/dev/nvme0n1", "476.9 GiB", NOW, "Samsung SSD 990 PRO, with Windows on it", True, False),
             ("/dev/sda", "931.5 GiB", [("data", 931, GUTTER)], "WDC WD10EZEX, one ext4 partition", False, False),
             ("/dev/sdb", "28.6 GiB", [("", 28, RULE)], "the install media: cannot be chosen", False, True)]
    for name, size, parts, what, chosen, locked in disks:
        if chosen:
            p.put(p.y, p.x, "▌", p.accent)
            p.put(p.y + 1, p.x, "▌", p.accent)
            p.put(p.y + 2, p.x, "▌", p.accent)
        p.put(p.y, p.x + 2, name, GUTTER if locked else PAPER, bold=chosen)
        p.put(p.y, p.x + p.w - len(size), size, GUTTER if locked else PAPER, bold=chosen)
        p.diskbar(parts, sum(part[1] for part in parts), x=p.x + 2, width=p.w - 2, row=p.y + 1)
        p.put(p.y + 2, p.x + 2, what, GUTTER if locked else LICHEN)
        p.y += 4

    p = page("disk-plan", "2 System", "The two answers as the slots they leave. One sentence each.", dp(2, 2, keys=[("←/→", "choose"), ("Enter", "go on"), ("q", "back")]))
    p.heading(2, "How is the system updated?", "This decides which partitions the system needs.")
    w = (p.w - 3) // 2
    for i, (title, tag, chosen, boxes, text) in enumerate([
        ("A/B", "default", True, [("A", "running", True), ("B", "spare", False)], "An update goes into the other slot and is tried once. If it does not start, it is never kept."),
        ("Mutable", "", False, [("root", "running", True)], "One root you change in place. An update cannot be undone."),
    ]):
        x = p.x + i * (w + 3)
        p.put(p.y, x, ("● " if chosen else "○ ") + title, p.accent if chosen else PAPER, bold=True)
        if tag:
            p.put(p.y, x + w - len(tag), tag, LICHEN)
        col = x
        for label, state, running in boxes:
            bw = (w - 1) // 2 if len(boxes) > 1 else w
            colour = ((p.accent if chosen else MOSS) if running else RULE)
            p.card(col, p.y + 2, bw, 3, colour)
            p.put(p.y + 2, col + 2, label, NIGHT if running else PAPER, bold=True)
            p.put(p.y + 3, col + 2, state, NIGHT if running else LICHEN)
            col += bw + 1
        for j, line in enumerate(wrap(text, w)):
            p.put(p.y + 6 + j, x, line, MOSS if chosen else LICHEN)
    p.y += 10

    p = page("disk-plan", "3 Disk use", "The disk now, and the disk each answer leaves. What happens to Windows is said once, beside each.", dp(3, 3, keys=[("↑/↓", "choose"), ("Enter", "go on"), ("q", "back")]))
    p.heading(3, "What happens to the disk?", "Keep what is on it, or give it all to the new system.")
    p.diskbar(NOW, 476, label="Now")
    p.gap(2)
    for title, chosen, parts, lost, tone in [
        ("Keep some of it", True, after(p), "Windows stays, 175.9 GiB smaller.", LEAF),
        ("Erase the whole disk", False, [("", 4, MOSS), ("A", 30, MOSS), ("B", 30, MOSS), ("state", 412, MOSS)], "Windows and everything in it is gone.", SALMON),
    ]:
        p.put(p.y, p.x, ("● " if chosen else "○ ") + title, p.accent if chosen else PAPER, bold=True)
        p.put(p.y, p.x + p.w - len(lost), lost, tone)
        p.diskbar(parts, 476, row=p.y + 1)
        p.y += 4

    p = page("disk-plan", "4 Partitions", "The bar, the table, and what A/B still needs. The selected row is marked on the bar; its actions are in the keys.",
             dp(4, 4, keys=[("↑/↓", "select"), ("m", "use as"), ("r", "resize"), ("d", "remove"), ("u", "undo"), ("Enter", "go on")]))
    p.heading(4, "Place the new system", "Make room for the four partitions A/B needs.")
    p.diskbar(after(p), 476, selected=2)
    left = p.w - 25
    top = p.y
    rows = [["nvme0n1p1", "260 MiB", "EFI", "kept"], ["nvme0n1p3", "300.0 GiB", "Windows", "shrunk"], ["(new)", "30.0 GiB", "Slot A", "new"],
            ["(new)", "30.0 GiB", "Slot B", "new"], ["(new)", "115.9 GiB", "State", "new"], ["nvme0n1p4", "750 MiB", "", "kept"]]
    p.table(["Partition", "Size", "Used as", "Change"], rows, aligns=["l", "r", "l", "l"], selected=2, colours=[GUTTER, SAND, None, None, None, GUTTER])
    nx = p.x + left + 2
    p.card(nx, top, 23, 7)
    p.put(top + 1, nx + 2, "A/B needs", MOSS, INK, bold=True)
    for i, (need, got) in enumerate([("EFI", "kept"), ("Slot A", "30.0 GiB"), ("Slot B", "30.0 GiB"), ("State", "115.9 GiB")]):
        p.put(top + 2 + i, nx + 2, "✓", LEAF, INK)
        p.put(top + 2 + i, nx + 4, need, PAPER, INK)
        p.put(top + 2 + i, nx + 21 - len(got), got, LICHEN, INK)
    for r in range(top, top + 8):
        for c in range(p.x + left, nx):
            p.grid[r][c].update(ch=" ", bg=NIGHT)

    p = page("disk-plan", "5 Plan", "Now over after, then only what changes.", dp(5, 5))
    p.heading(5, "The plan", "What recpart would do. Nothing has been written yet.")
    p.diskbar(NOW, 476, label="Now")
    p.diskbar(after(p), 476, label="After")
    p.gap()
    p.table(["Partition", "Change", "Before", "After", "Use"],
            [["nvme0n1p3", "shrunk", "475.9 GiB ntfs", "300.0 GiB ntfs", "Windows"], ["nvme0n1p5", "created", "", "30.0 GiB ext4", "root_a  /"],
             ["nvme0n1p6", "created", "", "30.0 GiB ext4", "root_b"], ["nvme0n1p7", "created", "", "115.9 GiB ext4", "state  /var"]],
            colours=[SAND, None, None, None])
    p.note("Three other partitions are left as they are.")

    p = page("disk-plan", "6 Commands", "Only the run sheet.", dp(6, 6))
    p.heading(6, "The commands", "Exactly what the apply will run, in this order.")
    runsheet(p)

    p = page("disk-plan", "7 Checks", "One verdict, and what stands behind it.", dp(7, 7))
    p.heading(7, "The rehearsal", "recpart ran every command as a test, without writing.")
    p.panel("success", "recpart would apply to this disk")
    p.checks([("The disk", [("done", "not in use"), ("done", "not the install media"), ("done", "room for A/B")]),
              ("Windows", [("done", "the volume is clean"), ("done", "not hibernated"), ("done", "can shrink to 300.0 GiB")])])

    p = page("disk-plan", "8 Confirm", "The point of no return: before and after, one sentence of what it costs, and the word.",
             dp(8, 8, keys=[("A-Z", "type the word"), ("Bksp", "erase"), ("Enter", "write, once unlocked"), ("q", "back")]))
    p.heading(8, "Write to the disk?", "Everything before this was a rehearsal.", SALMON)
    p.diskbar(NOW, 476, label="Now")
    p.diskbar(after(p), 476, label="After")
    p.gap()
    p.text("Windows shrinks by 175.9 GiB and three partitions are created. Nothing is erased, but have a backup: an interrupted resize can lose it.")
    p.gap()
    p.wordslots("DESTROY", "DEST")

    p = page("disk-plan", "9 Apply, running", "The bar and the run sheet. The running command is the raised row.", dp(9, 9, keys=[("q", "wait for it")], right="writing", states={9: "running"}))
    p.heading(9, "Writing the disk", "Do not turn the machine off.")
    p.bar(5 / 10)
    p.gap()
    runsheet(p, upto=5)

    p = page("disk-plan", "9 Apply, stopped", "The run sheet with the failed row raised, and under it what it printed and where that leaves the disk.", dp(9, 9, right="stopped", states={9: "failed"}))
    p.heading("bang", "The apply stopped", "At command 6 of 10.", SALMON)
    runsheet(p, upto=5, failed=6)
    p.gap()
    p.text("mkfs.ext4: /dev/nvme0n1p6: Input/output error", SALMON)
    p.text("Windows is shrunk and intact. Slot A is formatted; slot B and state are not.", MOSS)
    p.note("Check the disk, then plan again from step 5.")

    p = page("disk-plan", "10 Done", "The disk you now have, with where each part is mounted, and one bar on.", dp(10, 10, right="/mnt"))
    p.heading("check", "The disk is ready", "Laid out and mounted. Windows is still there.", LEAF)
    spans = p.diskbar(after(p), 476)
    for (col, _), mount in zip(spans, ["/boot", "", "/mnt", "", "/var", ""]):
        p.put(p.y, col, mount, LICHEN)
    p.button("Copy the system")


# ====================================================================== base-system
BS = ["Overview", "Check", "Copy", "fstab", "Done"]
BS_VALUES = ["", "12 passed", "48,213 files", "3 entries", ""]


def bs(selected, done, keys=STEP_KEYS, right="/mnt", states=None):
    p = Deliberate()
    p.band("Base system", "Overview" if selected == 0 else "Done" if selected == 4 else f"step {selected} of 3")
    p.steps(3, max(0, min(3, done - 1)), selected - 1 if 0 < selected < 4 else None)
    steps = [(label, "done" if i < done else "ready" if i == done else "todo", BS_VALUES[i] if i < done else "") for i, label in enumerate(BS)]
    for i, state in (states or {}).items():
        steps[i] = (steps[i][0], state, steps[i][2])
    p.path(steps, selected)
    p.keys(keys, right)
    return p


CHECKS = [("The target", ["mounted", "empty", "writable", "room for the system"]), ("The image", ["found", "an EROFS image", "outside the target"]), ("This system", ["running as root", "kernel reads EROFS"])]


def base_system_pages():
    p = page("base-system", "Overview", "The image, an arrow, the target.", bs(0, 1))
    p.heading(None, "Copy the system onto the disk", "Check the target, copy the system to it, write down its mounts.")
    from_to(p, ("The live image", ["/run/live-rootfs.erofs", "1.4 GiB, LevitateOS 1.0"]), ("/mnt", ["/dev/nvme0n1p5, slot A", "30.0 GiB, empty"]))
    p.button("Check the target")

    p = page("base-system", "1 Check", "Room as a gauge, and the checks by name.", bs(1, 2))
    p.heading(1, "Can the target take it?", "Twelve checks before a single file is copied.")
    p.gauge(3.1, 29.2, "The system needs about 3.1 GiB of the 29.2 GiB free.")
    p.checks([(label, [("done", item) for item in items]) for label, items in CHECKS])
    p.button("Copy the system")

    p = page("base-system", "1 Check, refused", "The same checks, with the one that failed marked, and the fix above them.", bs(1, 1, states={1: "failed"}, keys=[("r", "check again"), ("q", "back")]))
    p.heading("bang", "The target cannot take it", "Nothing was copied.", SALMON)
    p.text("/mnt is not empty: it holds etc, usr and var.", SALMON)
    p.text("Format it again in the disk step, or empty it by hand.", MOSS)
    p.gap()
    p.checks([(label, [("failed" if item == "empty" else "done", item) for item in items]) for label, items in CHECKS])

    p = page("base-system", "2 Copy, running", "How far, three rows tall. Four numbers and a bar.", bs(2, 2, keys=[("q", "wait for it")], right="copying", states={2: "running"}))
    p.heading(2, "Copying the system", "It cannot be stopped half-way.")
    p.gap()
    p.figure("62%", x=p.x, row=p.y)
    top = p.y
    p.y = top - 1
    p.facts([("Files", "29,892 of 48,213"), ("Written", "1.9 of about 3.1 GiB"), ("Elapsed", "0:25"), ("Left", "about 0:16")], x=p.x + 26)
    p.y = top + 5
    p.bar(0.62)

    p = page("base-system", "3 fstab", "The file as it will be written.", bs(3, 3))
    p.heading(3, "Write down the mounts", "So the installed system mounts the same filesystems when it boots.")
    p.filediff("/mnt/etc/fstab", [(" ", 1, "# /etc/fstab: static file system information"), ("-", 2, "UUID=0b9c6f4e-old   /      ext4  defaults  0 1"),
                                    ("+", 2, "UUID=5f1d2c3a-root  /      ext4  relatime  0 1"), (" ", 3, ""), ("-", 4, "UUID=7A3E-OLD       /boot  vfat  defaults  0 2"),
                                    ("+", 4, "UUID=1C2B-3A4D      /boot  vfat  relatime,fmask=0022,dmask=0022  0 0"), (" ", 5, "tmpfs               /tmp   tmpfs defaults  0 0"),
                                    ("+", 6, "UUID=9e8d7c6b-state /var   ext4  relatime  0 2")])
    p.button("Write the fstab")

    p = page("base-system", "3 fstab, asking", "Replacing lines asks first: the question, and only the lines it is about.", bs(3, 3, keys=CONFIRM_KEYS))
    p.heading(3, "Write down the mounts", "So the installed system mounts the same filesystems when it boots.")
    p.panel("warning", "Replace lines 2 and 4?", ["They describe mount points the new lines describe too."], keys=[("y", "replace them"), ("n", "keep the file")])
    p.filediff("/mnt/etc/fstab", [("-", 2, "UUID=0b9c6f4e-old   /      ext4  defaults  0 1"), ("+", 2, "UUID=5f1d2c3a-root  /      ext4  relatime  0 1"),
                                    ("-", 4, "UUID=7A3E-OLD       /boot  vfat  defaults  0 2"), ("+", 4, "UUID=1C2B-3A4D      /boot  vfat  relatime,fmask=0022,dmask=0022  0 0")])

    p = page("base-system", "Done", "Three figures and one bar on.", bs(4, 5))
    p.heading("check", "The system is on the disk", "Copied, and it knows its mounts.", LEAF)
    p.gap()
    w = p.w // 3
    for i, (figure, label) in enumerate([("48,213", "files"), ("3.1 GiB", "written"), ("0:41", "taken")]):
        p.put(p.y, p.x + i * w, figure, PAPER, bold=True)
        p.put(p.y + 1, p.x + i * w, label, LICHEN)
    p.button("Configure the system")


hub_pages()
disk_plan_pages()
base_system_pages()

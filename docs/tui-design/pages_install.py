"""The install: the hub, disk-plan and base-system. Second pass: every page is laid out
for the one thing it does, inside the frame all pages share."""
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
    """A check as a tile: its verdict, what was found, why it matters."""
    colour = TONES[tone]
    top = p.y
    p.card(x, top, w, 2 + len(lines), RAISED)
    for r in range(top, top + 2 + len(lines)):
        p.put(r, x, "█", colour)
    p.put(top, x + 2, f"{TONE_MARK[tone] if tone != 'danger' else '✗'} {title}", colour, RAISED, bold=True)
    for i, (text, tone_) in enumerate(lines):
        p.put(top + 1 + i, x + 2, text, tone_, RAISED)


def launch(p, hubpage=True):
    """What the hub shows for the step you are at: what the app will do, on what, and one
    bar to start it."""
    p.heading(3, "Copy the system onto the disk", "The longest step: a few minutes of copying, then the mounts are written down.")
    p.put(p.y, p.x, "WHAT IT DOES", GUTTER, bold=True)
    p.y += 1
    p.minipath([("Check the target", "ready"), ("Copy the system", "todo"), ("Write the fstab", "todo")])
    p.gap()
    half = (p.w - 5) // 2
    top = p.y
    p.card(p.x, top, half, 4)
    p.put(top, p.x + 2, "FROM", GUTTER, INK, bold=True)
    p.put(top + 1, p.x + 2, "The live image", PAPER, INK, bold=True)
    p.put(top + 2, p.x + 2, "/run/live-rootfs.erofs", LICHEN, INK)
    p.put(top + 3, p.x + 2, "1.4 GiB", LICHEN, INK)
    p.put(top + 1, p.x + half + 1, "──▶", p.accent, bold=True)
    p.card(p.x + half + 5, top, half, 4)
    p.put(top, p.x + half + 7, "TO", GUTTER, INK, bold=True)
    p.put(top + 1, p.x + half + 7, "/mnt", PAPER, INK, bold=True)
    p.put(top + 2, p.x + half + 7, "/dev/nvme0n1p5, slot A", LICHEN, INK)
    p.put(top + 3, p.x + half + 7, "30.0 GiB, empty", LICHEN, INK)
    p.y = top + 5
    p.button("Start base-system")
    p.hint(["The guide beside this has every command: Base System, steps 6-9."])


def hub_pages():
    p = page("install-hub", "1 Preflight", "The first page: each check is a tile that says what was found and why it matters, then the road ahead.", hub(0, 0))
    p.heading(1, "Can this machine take the install?", "Two things must hold before anything is touched.")
    half = (p.w - 2) // 2
    tile(p, p.x, half, "success", "Booted in UEFI mode", [("found /sys/firmware/efi/efivars", PAPER), ("systemd-boot needs UEFI", LICHEN)])
    tile(p, p.x + half + 2, half, "success", "Install media mounted", [("found /run/live-rootfs.erofs", PAPER), ("the system this installs", LICHEN)])
    p.y += 5
    p.put(p.y, p.x, "THE ROAD AHEAD", GUTTER, bold=True)
    p.y += 1
    p.minipath([("Disk", "ready"), ("Base system", "todo"), ("Configuration", "todo"), ("Boot", "todo"), ("Finish", "todo")])
    p.text("Nothing on any disk changes before you type DESTROY in the Disk step.", LICHEN)
    p.gap()
    p.button("Continue to the disk")

    p = page("install-hub", "1 Preflight, a check failed", "The failed tile takes the danger tone and carries the fix; there is no bar to press.", hub(0, 0))
    p.heading("bang", "This boot cannot take the install", "One of the two things does not hold.", SALMON)
    half = (p.w - 2) // 2
    tile(p, p.x, half, "danger", "Not booted in UEFI mode", [("no /sys/firmware/efi/efivars", PAPER), ("systemd-boot needs UEFI", LICHEN)])
    tile(p, p.x + half + 2, half, "success", "Install media mounted", [("found /run/live-rootfs.erofs", PAPER), ("the system this installs", LICHEN)])
    p.y += 5
    p.put(p.y, p.x, "WHAT TO DO", GUTTER, bold=True)
    p.y += 1
    p.text("Reboot, open the firmware's boot menu, and pick the UEFI entry for the install media.")

    page("install-hub", "3 A step ready to start", "The step you are at is a launch pad: what the app does, from what onto what, and one bar.", (lambda p: (launch(p), p)[1])(hub(2, 2)))

    p = page("install-hub", "2 A finished step", "A done step is a receipt: when, and what it left, drawn. Doing it again is there, quietly, with its cost.", hub(1, 3))
    p.heading(2, "The disk is laid out", "Done at 14:02. The steps after it build on this.")
    p.diskbar([("", 4, MOSS), ("Windows", 300, GUTTER), ("A", 30, p.accent), ("B", 30, LEAF), ("state", 112, SAND)], 476)
    p.gap()
    p.table(["", "Device", "Size", "Mounted at"], [["EFI", "nvme0n1p1", "260 MiB", "/mnt/boot"], ["Slot A", "nvme0n1p5", "30.0 GiB", "/mnt"], ["Slot B", "nvme0n1p6", "30.0 GiB", "for updates"], ["State", "nvme0n1p7", "115.9 GiB", "/mnt/var"]])
    p.gap()
    p.hint([("Enter",), " opens disk-plan again. That drops the base system and every step after it."])

    p = page("install-hub", "5 A step that waits", "A step ahead shows what it waits for, and already what it will write.", hub(4, 2))
    p.heading(5, "Install the boot loader", "systemd-boot and one boot entry for each slot.")
    p.put(p.y, p.x, "WAITS FOR", GUTTER, bold=True)
    p.y += 1
    p.minipath([("Base system", "ready"), ("Configuration", "todo"), ("Boot", "todo")])
    p.gap()
    p.put(p.y, p.x, "WHAT IT WILL WRITE", GUTTER, bold=True)
    p.y += 1
    p.card(p.x, p.y, p.w, 4)
    p.put(p.y, p.x + 2, "/boot  the EFI partition", MOSS, INK, bold=True)
    for i, (entry, note, colour) in enumerate([("LevitateOS (slot A)", "the default", p.accent), ("LevitateOS (slot B)", "empty until the first update", GUTTER)]):
        p.put(p.y + 1 + i, p.x + 4, "▸", colour, INK)
        p.put(p.y + 1 + i, p.x + 6, entry, PAPER, INK)
        p.put(p.y + 1 + i, p.x + 30, note, LICHEN, INK)
    p.put(p.y + 3, p.x + 4, "the kernel of slot A, and systemd-boot itself", LICHEN, INK)
    p.y += 5
    p.commands(["recsetup apply --root /mnt --boot ab --slot-a /dev/nvme0n1p5 --slot-b /dev/nvme0n1p6"])

    p = page("install-hub", "6 Finish", "The last page is the machine you made, one reminder, and one bar.", hub(5, 5))
    p.heading("check", "Ready to reboot", "The install is complete. This is the system it made.", LEAF)
    p.card(p.x, p.y, p.w, 6)
    p.fill(p.y, p.x, p.w, p.accent)
    p.put(p.y, p.x + 2, "studio", NIGHT, p.accent, bold=True)
    p.put(p.y, p.x + p.w - 16, "LevitateOS 1.0", NIGHT, p.accent)
    top = p.y + 2
    p.y = top
    p.facts([("Logs in", "ann, an administrator"), ("Keyboard", "de"), ("Clock", "Europe/Amsterdam")], x=p.x + 2, bg=INK)
    p.y = top
    p.facts([("Disk", "nvme0n1, beside Windows"), ("Layout", "A/B, slot A"), ("Language", "en_US.UTF-8")], x=p.x + p.w // 2 + 3, bg=INK)
    p.y = top + 5
    p.panel("warning", "Remove the install media when the machine restarts", ["Otherwise it boots the installer again."])
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
    """The commands as a numbered run sheet, a phase to a gutter. `upto` commands are
    done and the next is running; `failed` is the number that failed."""
    n = 0
    for phase, lines in RUN:
        for i, line in enumerate(lines):
            n += 1
            state = "todo" if upto is None else "failed" if n == failed else "done" if n <= upto else "running" if n == upto + 1 and failed is None else "todo"
            p.put(p.y, p.x, phase if i == 0 else "", MOSS if state != "todo" or upto is None else GUTTER, bold=True)
            p.put(p.y, p.x + 10, "┃", HAIR)
            p.put(p.y, p.x + 12, str(n).rjust(2), GUTTER)
            mark, colour = MARKS[state]
            here = state in ("running", "failed")
            if here:
                p.fill(p.y, p.x + 15, p.w - 15, RAISED)
            p.put(p.y, p.x + 16, mark, colour or p.accent, RAISED if here else None, bold=here)
            p.put(p.y, p.x + 19, line, PAPER if state != "todo" or upto is None else LICHEN, RAISED if here else None, bold=here)
            p.y += 1


def disk_plan_pages():
    p = page("disk-plan", "Overview", "The app as a map: three stretches, and the one line that matters, where writing starts.", dp(0, 0, right="no disk yet"))
    p.heading("diamond", "Prepare the install disk", "Every command is shown before anything is written.")
    w = (p.w - 6) // 3
    for i, (title, steps_) in enumerate([("Choose", ["Disk", "System", "Disk use", "Partitions"]), ("Review", ["Plan", "Commands", "Checks"]), ("Apply", ["Confirm", "Apply", "Done"])]):
        x = p.x + i * (w + 3)
        p.card(x, p.y, w, 6)
        p.put(p.y, x + 2, f"{i + 1}", p.accent, INK, bold=True)
        p.put(p.y, x + 4, title, PAPER, INK, bold=True)
        for j, step in enumerate(steps_):
            p.put(p.y + 1 + j, x + 4, step, LICHEN, INK)
        if i < 2:
            p.put(p.y + 2, x + w + 1, "▶", GUTTER)
    p.y += 7
    safe = round(p.w * 8 / 10)
    p.put(p.y, p.x, "━" * safe, LEAF)
    p.put(p.y, p.x + safe, "┃", PAPER, bold=True)
    p.put(p.y, p.x + safe + 1, "━" * (p.w - safe - 1), SALMON)
    p.put(p.y + 1, p.x, "nothing is written", LEAF)
    p.put(p.y + 1, p.x + safe - 14, "type DESTROY ▲", PAPER, bold=True)
    p.put(p.y + 1, p.x + p.w - 7, "written", SALMON)
    p.y += 3
    p.button("Start: choose a disk")

    p = page("disk-plan", "1 Disk", "Each disk shows what is on it as a bar, so the right one is recognised, not guessed. The install media cannot be chosen.", dp(1, 1, keys=[("↑/↓", "select"), ("Enter", "choose"), ("R", "read again"), ("q", "back")], right="3 disks"))
    p.heading(1, "Which disk?", "Nothing is written before the Confirm step.")
    disks = [("/dev/nvme0n1", "Samsung SSD 990 PRO", "476.9 GiB", NOW, "Windows, 3 partitions", True, False),
             ("/dev/sda", "WDC WD10EZEX", "931.5 GiB", [("data", 931, GUTTER)], "one ext4 partition, 512 GiB used", False, False),
             ("/dev/sdb", "SanDisk Ultra", "28.6 GiB", [("LevitateOS live", 28, RULE)], "the install media: cannot be chosen", False, True)]
    for name, model, size, parts, what, chosen, locked in disks:
        bg = RAISED if chosen else None
        if chosen:
            p.card(p.x, p.y, p.w, 3, RAISED)
            for r in range(3):
                p.put(p.y + r, p.x, "█", p.accent)
        p.put(p.y, p.x + 2, name, GUTTER if locked else PAPER, bg, bold=not locked)
        p.put(p.y, p.x + 17, model, LICHEN if not locked else GUTTER, bg)
        p.put(p.y, p.x + p.w - 2 - len(size), size, GUTTER if locked else PAPER, bg, bold=chosen)
        p.diskbar(parts, sum(part[1] for part in parts), x=p.x + 2, width=p.w - 4, row=p.y + 1)
        p.put(p.y + 2, p.x + 2, what, SALMON if locked else LICHEN, bg)
        p.y += 4

    p = page("disk-plan", "2 System", "The choice that shapes the disk, shown as what each leaves on it: the slots, and the partitions.", dp(2, 2, keys=[("←/→", "choose"), ("Enter", "go on"), ("q", "back")]))
    p.heading(2, "How is the system updated?", "This decides which partitions the system needs.")
    w = (p.w - 3) // 2
    for i, (title, tag, chosen, boxes, flow, text, parts) in enumerate([
        ("A/B: two slots", "default", True, [("A", True), ("B", False)], "update ─▶ B, tried once",
         "Read-only. An update that does not start is never kept.", [("EFI", 1, MOSS), ("root A", 20, None), ("root B", 20, LEAF), ("state", 23, SAND)]),
        ("Mutable: one system", "", False, [("root", True)], "update in place",
         "One root you change. An update cannot be undone.", [("EFI", 1, MOSS), ("root", 63, None)]),
    ]):
        x = p.x + i * (w + 3)
        edge = p.accent if chosen else RULE
        p.card(x, p.y, w, 11, RAISED)
        for r in range(11):
            p.put(p.y + r, x, "█", edge)
        p.put(p.y, x + 2, ("● " if chosen else "○ ") + title, p.accent if chosen else PAPER, RAISED, bold=True)
        if tag:
            p.put(p.y, x + w - len(tag) - 4, f" {tag} ", NIGHT, p.accent, bold=True)
        col = x + 2
        for label, running in boxes:
            bw = 13 if len(boxes) > 1 else 27
            p.fill(p.y + 2, col, bw, (p.accent if chosen else MOSS) if running else RULE)
            p.fill(p.y + 3, col, bw, (p.accent if chosen else MOSS) if running else RULE)
            p.put(p.y + 2, col + 1, label, NIGHT if running else LICHEN, bold=True)
            p.put(p.y + 3, col + 1, "running" if running else "spare", NIGHT if running else LICHEN)
            col += bw + 1
        p.put(p.y + 5, x + 2, flow, MOSS, RAISED)
        for j, line in enumerate(wrap(text, w - 4)):
            p.put(p.y + 6 + j, x + 2, line, LICHEN, RAISED)
        p.diskbar([(n, s, c or (p.accent if chosen else MOSS)) for n, s, c in parts], 64, x=x + 2, width=w - 4, row=p.y + 9)
    p.y += 12
    p.hint(["A/B is what this live system was started with."])

    p = page("disk-plan", "3 Disk use", "Two futures for the same disk, each drawn under what is on it now. What is lost is said in so many words.", dp(3, 3, keys=[("←/→", "choose"), ("Enter", "go on"), ("q", "back")]))
    p.heading(3, "What happens to the disk?", "Keep what is on it, or give it all to the new system.")
    p.diskbar(NOW, 476, label="Now")
    p.gap()
    w = (p.w - 3) // 2
    for i, (title, tag, chosen, parts, lost, tone) in enumerate([
        ("Keep some of it", "dual boot", True, after(p), "Windows stays, 175.9 GiB smaller", LEAF),
        ("Erase the whole disk", "simplest", False, [("", 4, MOSS), ("A", 30, MOSS), ("B", 30, MOSS), ("state", 412, MOSS)], "Windows and its data are gone", SALMON),
    ]):
        x = p.x + i * (w + 3)
        p.card(x, p.y, w, 6, RAISED)
        for r in range(6):
            p.put(p.y + r, x, "█", p.accent if chosen else RULE)
        p.put(p.y, x + 2, ("● " if chosen else "○ ") + title, p.accent if chosen else PAPER, RAISED, bold=True)
        p.put(p.y, x + w - len(tag) - 4, f" {tag} ", NIGHT if chosen else LICHEN, p.accent if chosen else RULE, bold=chosen)
        p.put(p.y + 2, x + 2, "AFTER", GUTTER, RAISED, bold=True)
        p.diskbar(parts, 476, x=x + 2, width=w - 4, row=p.y + 3)
        p.put(p.y + 5, x + 2, lost, tone, RAISED)
    p.y += 7
    p.hint(["You place the new partitions on the next page."])

    p = page("disk-plan", "4 Partitions", "Editing is done on the picture: the selected stretch is marked on the bar, its actions are keys beside the table, and what A/B still needs is always in view.",
             dp(4, 4, keys=[("↑/↓", "select"), ("m", "use as"), ("r", "resize"), ("f", "filesystem"), ("d", "remove"), ("u", "undo"), ("Enter", "go on")]))
    p.heading(4, "Place the new system", "Shrink, keep or remove what is there, and make room for the four partitions A/B needs.")
    p.diskbar(after(p), 476, selected=2)
    left = p.w - 25
    top = p.y
    rows = [["nvme0n1p1", "260 MiB", "EFI", "kept"], ["nvme0n1p3", "300.0 GiB", "Windows", "shrunk"], ["(new)", "30.0 GiB", "Slot A", "new"],
            ["(new)", "30.0 GiB", "Slot B", "new"], ["(new)", "115.9 GiB", "State", "new"], ["nvme0n1p4", "750 MiB", "", "kept"]]
    p.table(["Partition", "Size", "Used as", "Change"], rows, aligns=["l", "r", "l", "l"], selected=2, colours=[None, SAND, LEAF, LEAF, LEAF, GUTTER])
    nx = p.x + left + 2
    p.card(nx, top, 23, 7)
    p.put(top, nx + 2, "A/B NEEDS", GUTTER, INK, bold=True)
    for i, (need, got) in enumerate([("EFI", "kept"), ("Slot A", "30.0 GiB"), ("Slot B", "30.0 GiB"), ("State", "115.9 GiB")]):
        p.put(top + 2 + i, nx + 2, "✓", LEAF, INK)
        p.put(top + 2 + i, nx + 4, need, PAPER, INK)
        p.put(top + 2 + i, nx + 21 - len(got), got, LICHEN, INK)
    # The selected row is cut to the table's width so the card stays clear of it.
    for r in range(top, top + 8):
        for c in range(p.x + left, nx):
            p.grid[r][c].update(ch=" ", bg=NIGHT)
    p.y += 1
    p.panel("success", "recpart accepts this layout", ["Enter goes on to the plan."])

    p = page("disk-plan", "5 Plan", "Now against after, one bar over the other, then only the partitions that change.", dp(5, 5))
    p.heading(5, "The plan", "What recpart would do to the disk. Nothing has been written yet.")
    p.diskbar(NOW, 476, label="Now")
    p.diskbar(after(p), 476, label="After")
    p.gap()
    p.table(["Partition", "Change", "Before", "After", "Use"],
            [["nvme0n1p3", "shrunk", "475.9 GiB ntfs", "300.0 GiB ntfs", "Windows"], ["nvme0n1p5", "created", "", "30.0 GiB ext4", "root_a  /"],
             ["nvme0n1p6", "created", "", "30.0 GiB ext4", "root_b"], ["nvme0n1p7", "created", "", "115.9 GiB ext4", "state  /var"]],
            colours=[SAND, LEAF, LEAF, LEAF])
    p.gap()
    p.put(p.y, p.x, "UNTOUCHED", GUTTER, bold=True)
    p.put(p.y, p.x + 11, "nvme0n1p1 EFI · nvme0n1p2 · nvme0n1p4 recovery", LICHEN)
    p.y += 2
    p.hint([("Enter",), " shows the exact commands."])

    p = page("disk-plan", "6 Commands", "A run sheet: numbered, a phase to a gutter, in the order they will run.", dp(6, 6))
    p.heading(6, "The commands", "Exactly what the apply will run, from recpart's dry run. 10 commands in 4 phases.")
    runsheet(p)

    p = page("disk-plan", "7 Checks", "The rehearsal: what recpart tried without writing, and that nothing refused.", dp(7, 7))
    p.heading(7, "The rehearsal", "recpart ran every step as a test, without writing.")
    p.panel("success", "recpart would apply to this disk", ["All 10 commands rehearsed without a refusal."])
    p.checks([("The disk", [("done", "not in use by anything"), ("done", "not the install media"), ("done", "room for A/B after shrinking")]),
              ("Windows", [("done", "the volume is clean"), ("done", "not hibernated"), ("done", "can shrink to 300.0 GiB")])])
    p.gap().hint([("Enter",), " goes on to the one step that writes."])

    p = page("disk-plan", "8 Confirm", "The point of no return looks like one: what is lost in plain words, the disk before and after, and a word typed a letter a cell.",
             dp(8, 8, keys=[("A-Z", "type the word"), ("Bksp", "erase"), ("Enter", "write, once unlocked"), ("q", "back")]))
    p.heading(8, "Write to the disk?", "Everything before this was a rehearsal. This writes.", SALMON)
    p.diskbar(NOW, 476, label="Now")
    p.diskbar(after(p), 476, label="After")
    p.gap()
    p.panel("danger", "Windows shrinks by 175.9 GiB; three partitions are created", ["Nothing is erased. Have a backup of Windows anyway: a resize that is interrupted can lose it."])
    p.put(p.y, p.x, "TYPE DESTROY", GUTTER, bold=True)
    p.y += 1
    p.wordslots("DESTROY", "DEST")

    p = page("disk-plan", "9 Apply, running", "A long job shows how far, as a figure and a bar, and exactly which command is running now.", dp(9, 9, keys=[("q", "wait for it")], right="writing", states={9: "running"}))
    p.heading(9, "Writing the disk", "Do not turn the machine off.")
    width = p.figure("50%", row=p.y - 4, x=p.x + p.w - 17)
    p.bar(5 / 10)
    p.gap()
    runsheet(p, upto=5)

    p = page("disk-plan", "9 Apply, stopped", "A failure points at the command that failed, shows what it printed, and says what state the disk is in.", dp(9, 9, right="stopped", states={9: "failed"}))
    p.heading("bang", "The apply stopped", "At command 6 of 10. The commands after it did not run.", SALMON)
    runsheet(p, upto=5, failed=6)
    p.gap()
    p.panel("danger", "mkfs.ext4: /dev/nvme0n1p6: Input/output error", ["Now: Windows is shrunk and intact; slot A is formatted, slot B and state are not.",
                                                                    "What to do: check the disk, then plan again from step 5."])

    p = page("disk-plan", "10 Done", "The ending shows the disk you now have, with where each part is mounted, and one bar to the next step.", dp(10, 10, right="/mnt"))
    p.heading("check", "The disk is ready", "Laid out and mounted. Windows is still there.", LEAF)
    spans = p.diskbar(after(p), 476)
    for (col, w), mount in zip(spans, ["/boot", "", "/mnt", "", "/var", ""]):
        if mount:
            p.put(p.y, col, mount, LICHEN)
    p.y += 2
    p.table(["", "Device", "Size", "Mounted at"], [["Slot A", "nvme0n1p5", "30.0 GiB", "/mnt"], ["EFI", "nvme0n1p1", "260 MiB", "/mnt/boot"], ["State", "nvme0n1p7", "115.9 GiB", "/mnt/var"], ["Slot B", "nvme0n1p6", "30.0 GiB", "kept for updates"]])
    p.gap()
    p.button("Continue: copy the system")


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
    p = page("base-system", "Overview", "The step in one picture: the image, an arrow, the target.", bs(0, 1))
    p.heading("diamond", "Copy the system onto the disk", "Check the target, copy the system to it, write down its mounts.")
    half = (p.w - 5) // 2
    top = p.y
    for i, (label, title, lines) in enumerate([("FROM", "The live image", ["/run/live-rootfs.erofs", "1.4 GiB, read-only EROFS", "LevitateOS 1.0"]), ("TO", "/mnt", ["/dev/nvme0n1p5, slot A", "30.0 GiB ext4, empty", "with /boot and /var under it"])]):
        x = p.x + i * (half + 5)
        p.card(x, top, half, 6)
        p.put(top, x + 2, label, GUTTER, INK, bold=True)
        p.put(top + 1, x + 2, title, PAPER, INK, bold=True)
        for j, line in enumerate(lines):
            p.put(top + 2 + j, x + 2, line, LICHEN, INK)
    p.put(top + 2, p.x + half + 1, "──▶", p.accent, bold=True)
    p.y = top + 7
    p.minipath([("Check the target", "ready"), ("Copy the system", "todo"), ("Write the fstab", "todo")])
    p.gap()
    p.button("Start with the check")

    p = page("base-system", "1 Check", "Room is a gauge; the twelve checks are named, in three groups, each with its tick.", bs(1, 2))
    p.heading(1, "Can the target take it?", "Twelve checks before a single file is copied.")
    p.gauge(3.1, 29.2, "The system needs about 3.1 GiB of the 29.2 GiB free on /mnt.")
    p.checks([(label, [("done", item) for item in items]) for label, items in CHECKS])
    p.button("Copy the system")

    p = page("base-system", "1 Check, refused", "The check that failed is marked where it stands among the others, with recstrap's fix above them.", bs(1, 1, states={1: "failed"}, keys=[("r", "check again"), ("q", "back")]))
    p.heading("bang", "The target cannot take it", "One of twelve checks failed. Nothing was copied.", SALMON)
    p.panel("danger", "/mnt is not empty", ["It holds 3 entries: etc, usr, var. What to do: format it again in the disk step, or empty it by hand."])
    p.checks([(label, [("failed" if item == "empty" else "done", item) for item in items]) for label, items in CHECKS])
    p.hint([("r",), " checks again once it is fixed."])

    p = page("base-system", "2 Copy, running", "The one thing to watch is how far: a figure three rows tall, a bar, and four numbers.", bs(2, 2, keys=[("q", "wait for it")], right="copying", states={2: "running"}))
    p.heading(2, "Copying the system", "From the live image to /mnt. It cannot be stopped half-way.")
    p.figure("62%", x=p.x, row=p.y)
    top = p.y
    p.y = top
    p.facts([("Files", "29,892 of 48,213"), ("Written", "1.9 of about 3.1 GiB"), ("Elapsed", "0:25"), ("Left", "about 0:16")], x=p.x + 24)
    p.y = top + 5
    p.bar(0.62)
    p.gap()
    p.put(p.y, p.x, "KEPT AS THEY ARE", GUTTER, bold=True)
    p.y += 1
    col = p.x
    for chip in ["ownership", "permissions", "capabilities", "extended attributes"]:
        col = p.put(p.y, col, f" {chip} ", MOSS, RAISED) + 2

    p = page("base-system", "3 fstab", "The file as it will be written: lines going out in salmon, lines coming in in green, the rest quiet.", bs(3, 3))
    p.heading(3, "Write down the mounts", "So the installed system mounts the same filesystems when it boots.")
    p.filediff("/mnt/etc/fstab", [(" ", 1, "# /etc/fstab: static file system information"), ("-", 2, "UUID=0b9c6f4e-old   /      ext4  defaults  0 1"),
                                    ("+", 2, "UUID=5f1d2c3a-root  /      ext4  relatime  0 1"), (" ", 3, ""), ("-", 4, "UUID=7A3E-OLD       /boot  vfat  defaults  0 2"),
                                    ("+", 4, "UUID=1C2B-3A4D      /boot  vfat  relatime,fmask=0022,dmask=0022  0 0"), (" ", 5, "tmpfs               /tmp   tmpfs defaults  0 0"),
                                    ("+", 6, "UUID=9e8d7c6b-state /var   ext4  relatime  0 2")])
    p.gap()
    p.runs(p.y, p.x, [("+ 3 ", LEAF, True), ("written   ", LICHEN), ("− 2 ", SALMON, True), ("replaced   ", LICHEN), ("1 ", MOSS, True), ("kept", LICHEN)])
    p.y += 2
    p.button("Write the fstab")

    p = page("base-system", "3 fstab, asking", "Replacing lines asks first: the question on top, the same file under it.", bs(3, 3, keys=CONFIRM_KEYS))
    p.heading(3, "Write down the mounts", "So the installed system mounts the same filesystems when it boots.")
    p.panel("warning", "Replace lines 2 and 4?", ["They describe mount points the new lines describe too, most likely from an earlier run."], keys=[("y", "replace them"), ("n", "keep the file")])
    p.filediff("/mnt/etc/fstab", [("-", 2, "UUID=0b9c6f4e-old   /      ext4  defaults  0 1"), ("+", 2, "UUID=5f1d2c3a-root  /      ext4  relatime  0 1"),
                                    ("-", 4, "UUID=7A3E-OLD       /boot  vfat  defaults  0 2"), ("+", 4, "UUID=1C2B-3A4D      /boot  vfat  relatime,fmask=0022,dmask=0022  0 0")])

    p = page("base-system", "Done", "The ending: what is on the disk now, in four figures, and one bar on.", bs(4, 5))
    p.heading("check", "The system is on the disk", "Copied, and it knows its mounts.", LEAF)
    w = (p.w - 6) // 4
    for i, (figure, label) in enumerate([("48,213", "files"), ("3.1 GiB", "written"), ("0:41", "taken"), ("3", "fstab entries")]):
        x = p.x + i * (w + 2)
        p.card(x, p.y, w, 3)
        p.put(p.y, x + 2, figure, PAPER, INK, bold=True)
        p.put(p.y + 1, x + 2, label, LICHEN, INK)
    p.y += 4
    p.put(p.y, p.x, "NEXT", GUTTER, bold=True)
    p.y += 1
    p.text("Configuration: keyboard, time zone, language, hostname, root password and a user.", MOSS)
    p.gap()
    p.button("Continue: configure the system")


hub_pages()
disk_plan_pages()
base_system_pages()

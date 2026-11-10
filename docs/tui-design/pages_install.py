"""The install: the hub, disk-plan and base-system, page by page."""
from kit import *

PAGES = []


def page(app, name, note, p):
    PAGES.append({"app": app, "name": name, "note": note, "page": p})
    return p


STEP_KEYS = [("j/k", "step"), ("Enter", "go on"), ("q", "back")]
CONFIRM_KEYS = [("y", "yes"), ("n", "no")]

# ====================================================================== install hub
HUB_STEPS = ["Preflight", "Disk", "Base system", "Configuration", "Boot", "Finish"]
HUB_VALUES = ["UEFI", "vda · A/B", "48,213 files", "ann@studio", "A/B", ""]


def hub(selected, done, cols=100, rows=28, distro="levitate", right="state in /run/install"):
    p = Page(cols, rows, distro)
    p.band("Install", f"step {selected + 1} of 6")
    p.steps(6, done, selected)
    p.path([(label, "done" if i < done else "ready" if i == done else "todo", HUB_VALUES[i] if i < done else "")
            for i, label in enumerate(HUB_STEPS)], selected)
    p.keys([("j/k", "step"), ("Enter", "do it"), ("d", "mark done"), ("s", "shell"), ("q", "quit")], right)
    return p


def hub_pages():
    p = page("install-hub", "Preflight", "The first step: two checks, both passed. One key goes on.", hub(0, 0))
    p.heading(1, "Can this machine take the install?", "Two things are checked before anything else.")
    p.panel("success", "Ready to install", ["Both checks passed."])
    p.steplist([("", [("done", "Booted in UEFI mode"), ("done", "Install media mounted  /run/live-rootfs.erofs")])])
    p.gap().hint([("Enter",), " continues to the disk."])

    p = page("install-hub", "Preflight, a check failed", "A failed check says what to do; the step cannot be passed.", hub(0, 0))
    p.heading(1, "Can this machine take the install?", "Two things are checked before anything else.")
    p.panel("danger", "Not booted in UEFI mode", ["Reboot and pick the UEFI entry for the install media in the firmware's boot menu."])
    p.steplist([("", [("failed", "Booted in UEFI mode"), ("done", "Install media mounted  /run/live-rootfs.erofs")])])
    p.gap().hint(["Fix it, then start the install screen again."])

    p = page("install-hub", "A step ready to start", "The current step names the app it opens and what that app will do.", hub(2, 2))
    p.heading(3, "Copy the system onto the disk", "Check the target, copy the system, write its fstab.")
    p.panel("info", "Next: base-system", ["It shows a progress bar while it copies and asks before it replaces anything in the fstab."])
    p.section("Starts")
    p.commands(["base-system --then hub"])
    p.gap().hint([("Enter",), " starts it.   The guide beside this has it in full: Base System, steps 6-9."])

    p = page("install-hub", "A finished step", "A done step shows what it left, and warns that redoing it drops the steps after it.", hub(1, 3))
    p.heading(2, "Choose a disk and a layout", "Partition, format and mount the disk.")
    p.panel("success", "Done", facts=[("Disk", "/dev/vda"), ("Layout", "A/B"), ("Mounted at", "/mnt")])
    p.panel("warning", "Doing it again drops the steps after it", ["The base system and everything after it build on this disk layout."])
    p.section("Starts")
    p.commands(["disk-plan --then hub"])

    p = page("install-hub", "A step that waits", "A step ahead of the current one says what it waits for.", hub(4, 2))
    p.heading(5, "Install the boot loader", "systemd-boot and the boot entries for the disk's layout.")
    p.panel("info", "Waits for the steps before it", ["First: Base system."])
    p.section("Will run")
    p.commands(["recsetup apply --root /mnt --boot ab --slot-a /dev/vda2 --slot-b /dev/vda3 --machine-id"])

    p = page("install-hub", "Finish", "The last step: everything before it is ticked, one key reboots.", hub(5, 5))
    p.heading(6, "Reboot into the new system", "Unmount it and restart. Remove the install media when the machine restarts.")
    p.panel("success", "The install is complete", facts=[("System", "LevitateOS on /dev/vda"), ("Layout", "A/B, slot A"), ("Login", "ann, an administrator")])
    p.section("Will run")
    p.commands(["umount /mnt/var", "umount /mnt/boot", "umount /mnt", "reboot"])
    p.gap().hint([("Enter",), " unmounts and reboots."])


# ====================================================================== disk-plan
DP = ["Overview", "Disk", "System", "Disk use", "Plan", "Commands", "Checks", "Confirm", "Apply", "Done"]
DP_VALUES = ["", "vdz", "A/B", "erase", "4 parts", "11", "passed", "", "", ""]


def dp(selected, done, values=DP_VALUES, steps=DP, keys=STEP_KEYS, right="/dev/vdz"):
    p = Page()
    total = len(steps) - 1
    p.band("Disk", "Overview" if selected == 0 else f"step {selected} of {total}")
    p.steps(total, max(0, done - 1), selected - 1 if selected else None)
    p.path([(label, "done" if i < done else "ready" if i == done else "todo", values[i] if i < done else "")
            for i, label in enumerate(steps)], selected)
    p.keys(keys, right)
    return p


def disk_bar(p, parts, total):
    """A disk as one bar: each partition a stretch of it, then what each stretch is."""
    tints = [p.accent, LEAF, MOSS, p.highlight, GUTTER, SALMON]
    col = p.x
    for i, (name, size, _) in enumerate(parts):
        width = max(1, round(p.w * size / total))
        width = min(width, p.x + p.w - col)
        p.put(p.y, col, "█" * width, tints[i % len(tints)])
        col += width
    p.y += 1
    col = p.x
    for i, (name, size, label) in enumerate(parts):
        col = p.put(p.y, col, "■ ", tints[i % len(tints)])
        col = p.put(p.y, col, f"{name} ", PAPER)
        col = p.put(p.y, col, label, LICHEN) + 3
    p.y += 2


AB_PARTS = [("efi", 1, "1.0 GiB"), ("root_a", 20, "20.0 GiB"), ("root_b", 20, "20.0 GiB"), ("state", 23, "23.0 GiB")]


def disk_plan_pages():
    p = page("disk-plan", "Overview", "Where the step starts: what will happen, in three stretches, and what is chosen so far.", dp(0, 0, right="no disk yet"))
    p.heading("diamond", "Prepare the install disk", "Every command is shown before anything is written.")
    p.section("How it goes")
    for n, (title, pages_, what) in enumerate([
        ("Choose", "Disk · System · Disk use", "the disk, the layout, and whether to erase it or keep some"),
        ("Review", "Plan · Commands · Checks", "the plan, the exact commands, and whether recpart would refuse"),
        ("Apply", "Confirm · Apply · Done", "type DESTROY, watch it run, get the next step"),
    ]):
        col = p.put(p.y, p.x, f"{n + 1}", p.accent, bold=True)
        col = p.put(p.y, col + 2, title.ljust(8), PAPER, bold=True)
        p.put(p.y, col, pages_, MOSS)
        p.put(p.y + 1, p.x + 11, what, LICHEN)
        p.y += 3
    p.hint([("Enter",), " starts with the disk."])

    p = page("disk-plan", "1 Disk", "Disks as a list with the selection bar; the chosen one's details beside it.", dp(1, 1, right="3 disks"))
    p.heading(1, "Which disk?", "Nothing is written before the Confirm step.")
    list_w = 34
    top = p.y
    p.rows_list([("", "/dev/nvme0n1", "1.8 TiB", "NVMe"), ("", "/dev/sda", "476.9 GiB", "SATA"), ("", "/dev/sdb", "28.6 GiB", "USB ")], 0, list_w)
    p.y = top
    card_x, card_w = p.x + list_w + 2, p.w - list_w - 2
    for r in range(top, top + 9):
        p.fill(r, card_x, card_w, INK)
    p.put(top + 1, card_x + 2, "Samsung SSD 990 PRO", PAPER, INK, bold=True)
    p.put(top + 2, card_x + 2, "serial S6Z2NJ0W501942", LICHEN, INK)
    p.y = top + 4
    p.facts([("Size", "1.8 TiB"), ("Table", "gpt"), ("On it now", "Windows, 3 parts"), ("Live medium", "no")], x=card_x + 2, bg=INK)
    p.y = top + 11
    p.hint([("Enter",), " chooses it.   ", ("R",), " reads the disks again."])

    p = page("disk-plan", "2 System", "A choice between two is two cards; the chosen one carries the accent.", dp(2, 2))
    p.heading(2, "How is the system updated?", "This decides which partitions the system needs.")
    p.cards([
        ("A/B: two system slots", "default", ["The system is read-only, in one of two slots. An update goes into the other slot and is tried once; if it does not start, the machine goes back to the working one.", "", "EFI · root A · root B · state"], True),
        ("Mutable: one system", "", ["One root you change in place, as on most distributions. Updates cannot be tried first or rolled back.", "", "EFI · root"], False),
    ], height=9)
    p.hint([("←/→",), " chooses.   ", ("Enter",), " goes on.   A/B is what this live system was started with."])

    p = page("disk-plan", "3 Disk use", "The same two cards, then what is on the disk now.", dp(3, 3))
    p.heading(3, "What happens to the disk?", "Give it all to the new system, or keep some of what is on it.")
    p.cards([
        ("Erase the whole disk", "simplest", ["The A/B partitions take all of it. The disk is empty, so nothing is lost."], True),
        ("Keep some of it", "", ["Keep, shrink or delete partitions and place the new ones: for dual boot, or keeping data."], False),
    ], height=5)
    p.section("On /dev/vdz now", "64.0 GiB · no partition table")
    p.text("No partitions: the disk is empty.", LICHEN)

    p = page("disk-plan", "4 Plan", "The disk as one bar, then the partitions as a table. Nothing written yet.", dp(4, 4))
    p.heading(4, "The plan", "What recpart would do to the disk. Nothing has been written yet.")
    p.section("/dev/vdz", "64.0 GiB · A/B")
    disk_bar(p, AB_PARTS, 64)
    p.section("Partitions", "the whole disk is erased")
    p.table(["#", "Name", "Label", "Type", "Mount", "Size", "Change"],
            [["1", "efi", "EFI", "vfat", "/boot", "1.0 GiB", "created"], ["2", "root_a", "ROOT_A", "ext4", "/", "20.0 GiB", "created"],
             ["3", "root_b", "ROOT_B", "ext4", "none", "20.0 GiB", "created"], ["4", "state", "STATE", "ext4", "/var", "rest", "created"]],
            aligns=["r", "l", "l", "l", "l", "r", "l"])

    p = page("disk-plan", "5 Commands", "Every command of the apply, grouped, before any of them runs.", dp(5, 5))
    p.heading(5, "The commands", "Exactly what the apply will run, from recpart's dry run.")
    p.steplist([
        ("Partition", [("todo", "wipefs -a /dev/vdz"), ("todo", "sfdisk /dev/vdz"), ("todo", "udevadm settle --timeout=10")]),
        ("Format", [("todo", "mkfs.vfat -F 32 -n EFI /dev/vdz1"), ("todo", "mkfs.ext4 -F -L ROOT_A /dev/vdz2"), ("todo", "mkfs.ext4 -F -L ROOT_B /dev/vdz3"), ("todo", "mkfs.ext4 -F -L STATE /dev/vdz4")]),
        ("Mount", [("todo", "mount /dev/vdz2 /mnt"), ("todo", "mount /dev/vdz1 /mnt/boot"), ("todo", "mount /dev/vdz4 /mnt/var")]),
    ])

    p = page("disk-plan", "6 Checks", "One verdict in a panel, and where it came from.", dp(6, 6))
    p.heading(6, "Safety checks", "Whether recpart's dry run found a reason to refuse this disk.")
    p.panel("success", "recpart would apply to this disk", ["Its dry run went through every step without a refusal."])
    p.section("The dry run")
    p.commands(["recpart apply --disk /dev/vdz --mode ab --expect plan-9c4b6673.json --dry-run"])
    p.gap().hint([("Enter",), " goes on to the last step before anything is written."])

    p = page("disk-plan", "7 Confirm", "The one destructive moment: a danger panel, what is lost, and a word to type.", dp(7, 7, keys=[("Enter", "apply, once unlocked"), ("Bksp", "erase"), ("q", "back")]))
    p.heading(7, "Write to the disk?", "The last step before recpart writes anything.")
    p.panel("danger", "This erases everything on /dev/vdz", facts=[("Disk", "Fixture Disk 64G, serial FIXTURE0001"), ("Size", "64.0 GiB"), ("Becomes", "A/B: efi, root_a, root_b, state")])
    p.section("Type DESTROY to unlock")
    p.field("›", "DEST", "locked", width=40)
    p.gap().hint(["Backspace corrects a typo. Nothing is written until the word is whole and you press ", ("Enter",), "."])

    p = page("disk-plan", "8 Apply, running", "A long job: a thick bar, then each command with how it went.", dp(8, 8, keys=[("q", "wait for it")], right="writing /dev/vdz"))
    p.heading(8, "Writing the disk", "recpart is laying out /dev/vdz. Do not turn the machine off.")
    p.bar(6 / 11)
    p.gap()
    p.steplist([
        ("Partition", [("done", "wipefs -a /dev/vdz"), ("done", "sfdisk /dev/vdz"), ("done", "udevadm settle --timeout=10")]),
        ("Format", [("done", "mkfs.vfat -F 32 -n EFI /dev/vdz1"), ("done", "mkfs.ext4 -F -L ROOT_A /dev/vdz2"), ("running", "mkfs.ext4 -F -L ROOT_B /dev/vdz3"), ("todo", "mkfs.ext4 -F -L STATE /dev/vdz4")]),
        ("Mount", [("todo", "mount /dev/vdz2 /mnt"), ("todo", "mount /dev/vdz1 /mnt/boot")]),
    ])

    p = page("disk-plan", "8 Apply, failed", "A failure: the large mark turns to a warning, recpart's own message and fix, and where it stopped.", dp(8, 8, right="stopped"))
    p.heading("bang", "The apply stopped", "recpart stopped at the first command that failed.", SALMON)
    p.panel("danger", "mkfs.ext4 failed on /dev/vdz3 (E041)", ["What to do: check the disk for errors, then run the plan again from the Disk step."])
    p.steplist([
        ("Partition", [("done", "wipefs -a /dev/vdz"), ("done", "sfdisk /dev/vdz"), ("done", "udevadm settle --timeout=10")]),
        ("Format", [("done", "mkfs.vfat -F 32 -n EFI /dev/vdz1"), ("done", "mkfs.ext4 -F -L ROOT_A /dev/vdz2"), ("failed", "mkfs.ext4 -F -L ROOT_B /dev/vdz3"), ("todo", "mkfs.ext4 -F -L STATE /dev/vdz4")]),
    ])

    p = page("disk-plan", "9 Done", "Every finished app ends the same way: a large tick, what was done, and the next step.", dp(9, 9, right="/mnt"))
    p.heading("check", "Disk ready", "Laid out and mounted, ready for the system.", LEAF)
    p.panel("success", "Ready at /mnt", ["Installing into slot A; slot B stays unmounted for updates."])
    p.section("Mounted")
    p.table(["Path", "Device", "Use"], [["/mnt", "/dev/vdz2", "root, slot A"], ["/mnt/boot", "/dev/vdz1", "EFI"], ["/mnt/var", "/dev/vdz4", "state"]])
    p.gap().section("Next: base system")
    p.commands(["base-system"])
    p.gap().hint([("Enter",), " goes back to the install steps."])

    ed_steps = ["Overview", "Disk", "System", "Disk use", "Partitions", "Plan", "Commands", "Checks", "Confirm", "Apply", "Done"]
    ed_values = ["", "vdy", "A/B", "keep", "", "", "", "", "", "", ""]
    p = page("disk-plan", "4 Partitions, editing", "Keeping some of a disk: its bar, its partitions with one selected, what can be done to that one, and what A/B still needs.",
             dp(4, 4, ed_values, ed_steps, keys=[("↑/↓", "select"), ("m", "use as"), ("r", "resize"), ("d", "remove"), ("u", "undo"), ("Enter", "go on")], right="/dev/vdy"))
    p.heading(4, "Edit the partitions", "Nothing is written until the Confirm step.")
    disk_bar(p, [("vdy1", 1, "EFI"), ("vdy3", 150, "Windows"), ("new", 30, "slot A"), ("new", 30, "slot B"), ("new", 45, "state")], 256)
    p.table(["Partition", "Size", "Used as", "Change"],
            [["vdy1", "512.0 MiB", "EFI partition", "kept"], ["vdy3", "150.0 GiB", "", "shrunk"], ["(new)", "30.0 GiB", "System slot A", "new"],
             ["(new)", "30.0 GiB", "System slot B", "new"], ["(new)", "44.8 GiB", "State (/var)", "new"]],
            aligns=["l", "r", "l", "l"], selected=4, colours=[None, SAND, LEAF, LEAF, LEAF])
    p.gap()
    p.panel("success", "Ready: recpart accepts this layout", ["A/B has its EFI partition, both slots and the state partition."])


# ====================================================================== base-system
BS = ["Overview", "Check", "Copy", "fstab", "Done"]
BS_VALUES = ["", "12 passed", "48,213 files", "3 entries", ""]


def bs(selected, done, keys=STEP_KEYS, right="/mnt", states=None):
    p = Page()
    p.band("Base system", "Overview" if selected == 0 else "Done" if selected == 4 else f"step {selected} of 3")
    p.steps(3, max(0, min(3, done - 1)), selected - 1 if 0 < selected < 4 else None)
    steps = [(label, "done" if i < done else "ready" if i == done else "todo", BS_VALUES[i] if i < done else "") for i, label in enumerate(BS)]
    if states:
        for i, state in states.items():
            steps[i] = (steps[i][0], state, steps[i][2])
    p.path(steps, selected)
    p.keys(keys, right)
    return p


def base_system_pages():
    p = page("base-system", "Overview", "The same opening as every app: what it will do, and on what.", bs(0, 1))
    p.heading("diamond", "Copy the system onto the disk", "Check the target, copy the system to it, write its fstab.")
    p.section("Target")
    p.facts([("Mounted at", "/mnt"), ("Disk", "/dev/vda, A/B layout")])
    p.gap()
    p.table(["Mount point", "Device"], [["/mnt", "/dev/vda2"], ["/mnt/boot", "/dev/vda1"], ["/mnt/var", "/dev/vda4"]])
    p.gap().section("What happens")
    p.steplist([("", [("ready", "Check that /mnt is empty and mounted, with room for the system"), ("todo", "Copy the system image onto it"), ("todo", "Describe its mounts in its /etc/fstab")])])
    p.gap().hint([("Enter",), " starts with the check."])

    p = page("base-system", "1 Check", "A passed check: the verdict panel carries the figures.", bs(1, 2))
    p.heading(1, "Can the target take it?", "It must be an empty, mounted filesystem with room for the system.")
    p.panel("success", "Ready to copy", facts=[("Target", "/mnt"), ("System image", "/run/live-rootfs.erofs"), ("Image size", "1.4 GiB"), ("Free space", "40.0 GiB"), ("Checks", "12 passed")])
    p.hint([("Enter",), " goes on to the copy."])

    p = page("base-system", "1 Check, refused", "A refusal uses the same panel in the danger tone, with recstrap's own fix.", bs(1, 1, states={1: "failed"}))
    p.heading(1, "Can the target take it?", "It must be an empty, mounted filesystem with room for the system.")
    p.panel("danger", "/mnt is not empty (E012)", ["What to do: unmount it and format it again in the disk step, or empty it by hand."])
    p.section("What ran", "exit 12")
    p.commands(["recstrap /mnt --check --json"])
    p.gap().hint([("r",), " checks again once it is fixed."])

    p = page("base-system", "2 Copy, running", "The long step: a thick bar and four figures, nothing else.", bs(2, 2, keys=[("q", "wait for it")], right="copying", states={2: "running"}))
    p.heading(2, "Copying the system", "Ownership, permissions, capabilities and extended attributes are kept.")
    p.bar(0.62)
    p.gap()
    p.facts([("Files", "29,892 of 48,213"), ("Written", "1.9 GiB of about 3.1 GiB"), ("Elapsed", "0:25"), ("Left", "about 0:16")])
    p.gap().text("The copy cannot be stopped half-way.", LICHEN)

    p = page("base-system", "3 fstab", "A table to review before it is written; rows that would be replaced are tinted.", bs(3, 3))
    p.heading(3, "Write the fstab", "So the installed system mounts the same filesystems when it boots.")
    p.section("Entries", "3")
    p.table(["Mount point", "Device", "Type", "Boot check"], [["/", "UUID=5f1d2c3a-root", "ext4", "first"], ["/boot", "UUID=1C2B-3A4D", "vfat", "no"], ["/var", "UUID=9e8d7c6b-state", "ext4", "after /"]])
    p.gap().section("Already in /mnt/etc/fstab", "3 lines")
    p.table(["Line", "Mount point", "Device", "Type", ""], [["2", "/", "UUID=0b9c6f4e-old", "ext4", "replaced"], ["4", "/boot", "UUID=7A3E-OLD", "vfat", "replaced"], ["5", "/tmp", "tmpfs", "tmpfs", "kept"]],
            aligns=["r", "l", "l", "l", "l"], colours=[SAND, SAND, None])
    p.gap().hint([("Enter",), " writes the entries, replacing 2 lines."])

    p = page("base-system", "3 fstab, asking", "A yes-or-no is always a warning panel at the top of the page, with its two keys inside it.", bs(3, 3, keys=CONFIRM_KEYS))
    p.heading(3, "Write the fstab", "So the installed system mounts the same filesystems when it boots.")
    p.panel("warning", "Replace lines 2 and 4 of /mnt/etc/fstab?", ["They describe mount points the new entries describe too, most likely from an earlier run."], keys=[("y", "replace them"), ("n", "keep the file")])
    p.section("Entries", "3")
    p.table(["Mount point", "Device", "Type", "Boot check"], [["/", "UUID=5f1d2c3a-root", "ext4", "first"], ["/boot", "UUID=1C2B-3A4D", "vfat", "no"], ["/var", "UUID=9e8d7c6b-state", "ext4", "after /"]])

    p = page("base-system", "Done", "The shared ending: tick, what was done, next step.", bs(4, 5))
    p.heading("check", "Base system installed", "The system is on the disk and knows its mounts.", LEAF)
    p.panel("success", "Next: configuration", ["Keyboard, time zone, language, hostname, root password and a user."])
    p.section("What was done")
    p.facts([("Target", "/mnt"), ("Files copied", "48,213"), ("Written", "3.1 GiB in 0:41"), ("fstab", "/mnt/etc/fstab, 3 entries")])
    p.gap().hint([("Enter",), " goes back to the install steps."])


hub_pages()
disk_plan_pages()
base_system_pages()

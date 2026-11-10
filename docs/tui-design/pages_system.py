"""system-setup, recab, the guide, and the install screen that holds the hub and the guide.

Third pass: balance. Each page keeps the one picture of what it does, and loses whatever
competed with it."""
import pathlib
import re
from kit import *
from pages_install import PAGES as _INSTALL, hub, launch, STEP_KEYS, CONFIRM_KEYS

PAGES = []


def page(app, name, note, p):
    PAGES.append({"app": app, "name": name, "note": note, "page": p})
    return p


LIST_KEYS = [("↑/↓", "select"), ("Enter", "choose"), ("Bksp", "erase"), ("Esc", "cancel")]
TEXT_KEYS = [("Enter", "accept"), ("Bksp", "erase"), ("Esc", "cancel")]

# ---- the world, from the app's own land bitmap
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
FIELDS = [("Keyboard", "de"), ("Time zone", "Europe/Amsterdam"), ("Language", "en_US.UTF-8"), ("Hostname", "studio"), ("Root password", "set"), ("User", "ann, an administrator")]


def ss(selected, done, keys=STEP_KEYS, right="/mnt", distro="levitate", states=None):
    p = Deliberate(distro=distro)
    p.band("Configuration", "Overview" if selected == 0 else "Done" if selected == 8 else f"step {selected} of 7")
    p.steps(7, max(0, min(7, done - 1)), selected - 1 if 0 < selected < 8 else None)
    steps = [(label, "done" if i < done else "ready" if i == done else "todo", SS_VALUES[i] if i < done else "") for i, label in enumerate(SS)]
    for i, state in (states or {}).items():
        steps[i] = (steps[i][0], state, steps[i][2])
    p.path(steps, selected)
    p.keys(keys, right)
    return p


def machine_card(p, filled, numbered=False):
    """The machine as a card with six lines: empty at the start, whole at the end. The
    overview, the review and the ending are this one card."""
    p.card(p.x, p.y, p.w, 9)
    p.fill(p.y, p.x, p.w, p.accent if filled == 6 else RULE)
    p.put(p.y, p.x + 2, "studio" if filled >= 4 else "this machine", NIGHT if filled == 6 else PAPER, bold=True)
    p.put(p.y, p.x + p.w - 16, "LevitateOS 1.0", NIGHT if filled == 6 else LICHEN)
    for i, (label, value) in enumerate(FIELDS):
        x = p.x + 2 + (i // 3) * (p.w // 2)
        row = p.y + 2 + (i % 3) * 2
        done = i < filled
        if numbered:
            p.put(row, x, str(i + 1), GUTTER, INK)
            x += 3
        p.put(row, x, label, LICHEN, INK)
        p.put(row + 1, x, value if done else "·" * 12, PAPER if done else HAIR, INK, bold=done)
    p.y += 10


EUROPE = [("Amsterdam", "02:20", "+01:00", 52.4, 4.9), ("Athens", "03:20", "+02:00", 38.0, 23.7), ("Berlin", "02:20", "+01:00", 52.5, 13.4),
          ("Brussels", "02:20", "+01:00", 50.8, 4.3), ("Dublin", "01:20", "+00:00", 53.3, -6.3), ("Helsinki", "03:20", "+02:00", 60.2, 25.0),
          ("Lisbon", "01:20", "+00:00", 38.7, -9.1), ("London", "01:20", "+00:00", 51.5, -0.1), ("Madrid", "02:20", "+01:00", 40.4, -3.7),
          ("Oslo", "02:20", "+01:00", 59.9, 10.7), ("Paris", "02:20", "+01:00", 48.9, 2.3)]
LIST_W = 34
SIDE_H = 14


def side(p, top, height=SIDE_H):
    """The card beside a list or a field: what the answer would mean. One height, so
    every question's page has the same two columns."""
    x, w = p.x + LIST_W + 2, p.w - LIST_W - 2
    p.card(x, top, w, height)
    return x, w


def zone_page(distro="levitate"):
    p = ss(2, 2, LIST_KEYS, distro=distro)
    p.heading(2, "Where is this machine?", "The clock of the new system.")
    top = p.y
    p.field("Search", "eu", "64 of 598", width=LIST_W)
    p.gap()
    p.rows_list([("Europe/", city, clock, "") for city, clock, *_ in EUROPE], 0, LIST_W, match="eu")
    x, w = side(p, top)
    p.put(top + 1, x + 2, "Amsterdam", PAPER, INK, bold=True)
    p.put(top + 1, x + w - 8, "+01:00", LICHEN, INK)
    world(p, x + 2, top + 3, w - 4, 9, (52.4, 4.9), [(lat, lon) for *_, lat, lon in EUROPE])
    p.put(top + 12, x + 2, "02:20 there now", MOSS, INK)
    return p


def system_setup_pages():
    p = page("system-setup", "Overview", "The machine as a card with six empty lines.", ss(0, 1))
    p.heading(None, "Make this machine yours", "Six questions. Nothing changes until you have seen the commands.")
    machine_card(p, 0)
    p.button("Start with the keyboard")

    p = page("system-setup", "1 Keyboard", "The list, and the keys you would get. The two keys that differ from the layout before are marked.", ss(1, 1, LIST_KEYS + [("Tab", "try it")]))
    p.heading(1, "Which keyboard do you type on?", "It is loaded on this console as you move.")
    top = p.y
    p.field("Search", "de", "4 of 22", width=LIST_W)
    p.gap()
    p.rows_list([("", "de", "", ""), ("", "de-latin1", "", ""), ("", "de-latin1-nodeadkeys", "", ""), ("", "de_CH-latin1", "", "")], 0, LIST_W, match="de")
    x, w = side(p, top)
    p.put(top + 1, x + 2, "German", PAPER, INK, bold=True)
    p.put(top + 1, x + w - 8, "QWERTZ", LICHEN, INK)
    for r, keys in enumerate(["q w e r t z u", "a s d f g h j", "y x c v b n m"]):
        col = x + 2 + r
        for key in keys.split():
            odd = key in "zy"
            p.put(top + 3 + r * 2, col, f" {key} ", NIGHT if odd else PAPER, p.highlight if odd else RAISED, bold=True)
            col += 4
    p.fill(top + 11, x + 2, w - 4, RAISED)
    p.put(top + 11, x + 3, "zwölf yaks", PAPER, RAISED, bold=True)
    p.grid[top + 11][x + 13]["bg"] = p.accent

    page("system-setup", "2 Time zone", "The list with each zone's clock, and the place on the map.", zone_page())

    p = page("system-setup", "3 Language", "The list, and how the system would write today.", ss(3, 3, LIST_KEYS))
    p.heading(3, "Which language?", "The system's language and how it writes dates and numbers.")
    top = p.y
    p.field("Search", "en", "2 of 16", width=LIST_W)
    p.gap()
    p.rows_list([("", "en_GB.UTF-8", "", ""), ("", "en_US.UTF-8", "", "")], 1, LIST_W, match="en")
    x, w = side(p, top)
    p.put(top + 1, x + 2, "English", PAPER, INK, bold=True)
    p.put(top + 1, x + w - 15, "United States", LICHEN, INK)
    for i, value in enumerate(["Tuesday, November 10, 2026", "2:20 AM", "1,234,567.89", "the week starts on Sunday"]):
        p.put(top + 3 + i * 2, x + 2, value, MOSS if i == 3 else PAPER, INK)

    p = page("system-setup", "4 Hostname", "The field, and the name where it will appear.", ss(4, 4, TEXT_KEYS))
    p.heading(4, "What is this machine called?", "Its name on the prompt and on the network.")
    top = p.y
    p.field("Hostname", "studio", width=LIST_W)
    x, w = side(p, top)
    p.runs(top + 2, x + 2, [("studio", p.accent, True), (" login:", MOSS)], INK)
    p.runs(top + 5, x + 2, [("ann@", MOSS), ("studio", p.accent, True), (" ~ $", MOSS)], INK)
    p.runs(top + 8, x + 2, [("studio", p.accent, True), (".local", MOSS)], INK)
    p.put(top + 9, x + 2, "on the network", GUTTER, INK)

    p = page("system-setup", "4 Hostname, refused", "The character that breaks the rule is marked in the field; recsetup's reason is under it. The preview stays empty.", ss(4, 4, TEXT_KEYS))
    p.heading(4, "What is this machine called?", "Its name on the prompt and on the network.")
    top = p.y
    p.field("Hostname", "studio_pc", width=LIST_W)
    p.put(p.y - 1, p.x + 17, "_", NIGHT, SALMON, bold=True)
    p.gap()
    for line in wrap("An underscore is not allowed: letters, digits and hyphens only.", LIST_W):
        p.put(p.y, p.x, line, SALMON)
        p.y += 1
    x, w = side(p, top)
    p.runs(top + 2, x + 2, [("············", HAIR), (" login:", GUTTER)], INK)

    p = page("system-setup", "5 Root password", "Two entries, a strength line, and what the choice means.", ss(5, 5, TEXT_KEYS))
    p.heading(5, "A password for root?", "The system's own administrator account.")
    top = p.y
    p.field("Password ", "••••••••••", width=LIST_W, cursor=False)
    p.meter(0.75, "", width=LIST_W)
    p.gap()
    p.field("Once more", "••••••", width=LIST_W)
    p.gap()
    p.put(p.y, p.x, "Typed on the de keyboard.", GUTTER)
    x, w = side(p, top)
    p.put(top + 1, x + 2, "With a password", PAPER, INK, bold=True)
    for i, line in enumerate(wrap("You can log in as root on the console, and repair the system when nothing else works.", w - 4)):
        p.put(top + 2 + i, x + 2, line, MOSS, INK)
    p.put(top + 7, x + 2, "Left empty", PAPER, INK, bold=True)
    for i, line in enumerate(wrap("Root cannot log in. Only your own user administers, with sudo.", w - 4)):
        p.put(top + 8 + i, x + 2, line, MOSS, INK)

    p = page("system-setup", "6 User", "The field, and the account as it will exist.", ss(6, 6, TEXT_KEYS))
    p.heading(6, "Who uses this machine?", "Your own account. Leave the name empty to create none.")
    top = p.y
    p.field("User name", "ann", width=LIST_W)
    x, w = side(p, top)
    for r in range(3):
        p.fill(top + 1 + r, x + 2, 7, p.accent)
    p.put(top + 2, x + 5, "A", NIGHT, p.accent, bold=True)
    p.put(top + 1, x + 11, "ann", PAPER, INK, bold=True)
    p.put(top + 2, x + 11, "administrator", MOSS, INK)
    p.put(top + 3, x + 11, "may use sudo", LICHEN, INK)
    for i, (label, value) in enumerate([("Home", "/home/ann"), ("Shell", "/bin/bash"), ("Groups", "ann, wheel")]):
        p.put(top + 6 + i * 2, x + 2, label, LICHEN, INK)
        p.put(top + 6 + i * 2, x + 11, value, PAPER, INK)

    p = page("system-setup", "7 Apply, reviewing", "The card, filled. A number before each line: that key changes it.", ss(7, 7, keys=[("1-6", "change an answer"), ("space", "the commands"), ("Enter", "apply"), ("q", "back")]))
    p.heading(7, "Is this right?", "Nothing has changed yet.")
    machine_card(p, 6, numbered=True)
    p.button("Apply to /mnt")

    p = page("system-setup", "7 Apply, the commands", "The same page turned over: only the commands.", ss(7, 7, keys=[("space", "the answers"), ("Enter", "apply"), ("q", "back")]))
    p.heading(7, "Is this right?", "Nothing has changed yet. Six commands, each inside the new system.")
    for what, command in [("Time zone", "recipe-system-setup timezone Europe/Amsterdam"), ("Language", "recipe-system-setup locale en_US.UTF-8"), ("Keyboard", "recipe-system-setup keymap de"),
                          ("Hostname", "recipe-system-setup hostname studio"), ("User", "recipe-system-setup user ann /bin/bash wheel"), ("Passwords", "chpasswd")]:
        p.put(p.y, p.x, what, MOSS)
        p.put(p.y, p.x + 12, "○", GUTTER)
        p.put(p.y, p.x + 15, command, PAPER)
        p.y += 1
    p.button("Apply to /mnt")

    p = page("system-setup", "7 Apply, asking", "The yes-or-no, over the card it is about.", ss(7, 7, CONFIRM_KEYS))
    p.heading(7, "Is this right?", "Nothing has changed yet.")
    p.panel("warning", "Apply these six answers to /mnt?", keys=[("y", "apply"), ("n", "not yet")])
    machine_card(p, 6)

    p = page("system-setup", "7 Apply, stopped", "The commands with the failed one raised, what it printed, and the bar to try again.", ss(7, 7, right="stopped", states={7: "failed"}))
    p.heading("bang", "The configuration stopped", "At command 5 of 6.", SALMON)
    for what, command, state in [("Time zone", "timezone Europe/Amsterdam", "done"), ("Language", "locale en_US.UTF-8", "done"), ("Keyboard", "keymap de", "done"),
                                 ("Hostname", "hostname studio", "done"), ("User", "user ann /bin/bash wheel", "failed"), ("Passwords", "chpasswd", "todo")]:
        here = state == "failed"
        if here:
            p.fill(p.y, p.x + 11, p.w - 11, RAISED)
        p.put(p.y, p.x, what, GUTTER if state == "todo" else MOSS)
        mark, colour = MARKS[state]
        p.put(p.y, p.x + 12, mark, colour, RAISED if here else None, bold=here)
        p.put(p.y, p.x + 15, command, LICHEN if state == "todo" else PAPER, RAISED if here else None, bold=here)
        p.y += 1
    p.gap()
    p.text("useradd: cannot lock /etc/passwd; try again later", SALMON)
    p.text("The four commands before it are done and are safe to run again.", MOSS)
    p.button("Apply again")

    p = page("system-setup", "Done", "The card once more, now true.", ss(8, 9))
    p.heading("check", "This machine is yours", "It knows its keyboard, clock, language and name, and who may log in.", LEAF)
    machine_card(p, 6)
    p.button("Make it boot")

    page("system-setup", "2 Time zone, AcornOS", "The same page on AcornOS: its band and its accent, nothing else differs.", zone_page("acorn"))


# ====================================================================== recab
def recab(selected, trial=False, keys=None, right=None, history=""):
    p = Deliberate()
    place = ["Overview", "Try slot B", "Commit", "Roll back", "Write a system", "Reboot", "History"][selected]
    p.band("Updates", place)
    seg = (p.cols - 2 - 1) // 2
    for i, (name, state) in enumerate([("Slot A", "running"), ("Slot B", "next" if trial else "idle")]):
        colour = {"running": p.accent, "next": p.highlight, "idle": RULE}[state]
        p.fill(2, 1 + i * (seg + 1), seg, colour)
        p.put(2, 3 + i * (seg + 1), name, NIGHT if state != "idle" else LICHEN, bold=state != "idle")
    p.menu([
        (None, [("Overview", "none", "o")]),
        ("Changes", [("Try slot B", "ready", "t"), ("Commit slot A", "todo", "c"), ("Roll back to B", "ready", "x"), ("Write to slot B", "ready", "s"), ("Reboot", "ready", "R")]),
        (None, [("History" + history, "none", "H")]),
    ], selected)
    p.keys(keys or [("j/k", "select"), ("Enter", "run it"), ("r", "read again"), ("q", "quit")], right or "running slot A")
    return p


def two_slots(p, a, b, h=5):
    """The two slots as blocks, side by side: the picture every recab page is about."""
    w = (p.w - 3) // 2
    p.slotbox(p.x, p.y, w, h, "A", a[1], a[2], a[0])
    p.slotbox(p.x + w + 3, p.y, w, h, "B", b[1], b[2], b[0])
    p.y += h + 1


def fork(p, bg=None):
    """A trial boot as a fork: one line in, two ways out."""
    p.put(p.y, p.x, "reboot ─▶", LICHEN, bg)
    p.put(p.y, p.x + 10, " slot B, once ", NIGHT, p.highlight, bold=True)
    p.put(p.y, p.x + 25, "─┬─ starts healthy ─▶", GUTTER, bg)
    p.put(p.y, p.x + 47, "B is kept", LEAF, bg, bold=True)
    p.put(p.y + 1, p.x + 26, "└─ does not ──────▶", GUTTER, bg)
    p.put(p.y + 1, p.x + 47, "back to slot A", PAPER, bg, bold=True)
    p.y += 3


def recab_pages():
    p = page("recab", "Overview", "The two slots, and one line: what boots next.", recab(0))
    p.heading(None, "Two systems, one running", "An update is written to the other slot and tried once before it is kept.")
    two_slots(p, ("running", [("LevitateOS 1.0", PAPER), ("known-good", MOSS)], "running"), ("idle", [("the previous system", PAPER), ("last trial failed, 5 Nov", MOSS)], ""))
    p.runs(p.y, p.x, [("Next boot  ", LICHEN), ("slot A", PAPER, True), ("      Roll back goes to  ", LICHEN), ("slot B", PAPER, True)])

    p = page("recab", "Overview, a trial pending", "Slot B takes the second accent everywhere at once, and the one line becomes the fork.", recab(0, trial=True))
    p.heading(None, "Slot B boots next, once", "A trial: kept if it starts healthy, undone if it does not.")
    two_slots(p, ("running", [("LevitateOS 1.0", PAPER), ("known-good", MOSS)], "running"), ("next", [("LevitateOS 1.1", PAPER), ("written today, never booted", MOSS)], "next boot"))
    fork(p)
    p.button("Reboot into the trial", key="R")

    p = page("recab", "Try slot B", "Before a change: the fork it sets up, and the bar.", recab(1))
    p.heading(None, "Try slot B on the next boot", "Boot it once and see. Slot A stays as it is.")
    p.gap()
    fork(p)
    p.gap()
    p.commands(["recab set-next B"])
    p.button("Set slot B for the next boot")

    p = page("recab", "Try slot B, asking", "The yes-or-no, over the same fork.", recab(1, keys=CONFIRM_KEYS))
    p.heading(None, "Try slot B on the next boot", "Boot it once and see. Slot A stays as it is.")
    p.panel("warning", "Run recab set-next B?", keys=[("y", "run it"), ("n", "cancel")])
    fork(p)

    p = page("recab", "Try slot B, done", "After a change: the slots as they now are, and the bar to act on it.", recab(1, trial=True, history=" · 1"))
    p.heading("check", "Slot B boots next, once", "Reboot to start the trial.", LEAF)
    two_slots(p, ("running", [("stays the default", MOSS), ("stays known-good", MOSS)], "running"), ("next", [("boots once", PAPER), ("kept only if healthy", MOSS)], "next boot"))
    p.button("Reboot into the trial", key="R")

    p = page("recab", "Roll back, refused", "A change recab would refuse says so before you press anything, with the reason on the slot it is about.", recab(3))
    p.heading("bang", "Nothing to roll back to", "recab would refuse this now.", SALMON)
    two_slots(p, ("running", [("committed", MOSS), ("the only known-good system", MOSS)], "running"), ("idle", [("written today, never booted", PAPER), ("not known-good", SALMON)], ""))
    p.text("A roll back goes to a system that has already run well. Try slot B first.", MOSS)

    p = page("recab", "Write a system, choosing", "Where the new system comes from is a choice from what is plugged in.", recab(4, keys=[("↑/↓", "select"), ("Enter", "choose"), ("Esc", "cancel")]))
    p.heading(None, "Write a new system to slot B", "Your accounts, hostname, network and services are carried over.")
    for title, where, parts, chosen in [("LevitateOS 1.1 release media", "/dev/sr0", "image, kernel and initramfs found", True),
                                        ("LevitateOS 1.0 release media", "/dev/sdb", "the version slot A runs", False), ("An image file…", "", "choose the three files by hand", False)]:
        if chosen:
            p.card(p.x, p.y, p.w, 2, RAISED)
            p.put(p.y, p.x, "▌", p.accent, RAISED)
            p.put(p.y + 1, p.x, "▌", p.accent, RAISED)
        bg = RAISED if chosen else None
        p.put(p.y, p.x + 2, title, PAPER, bg, bold=chosen)
        p.put(p.y, p.x + p.w - 2 - len(where), where, LICHEN, bg)
        p.put(p.y + 1, p.x + 2, parts, MOSS if chosen else LICHEN, bg)
        p.y += 3

    p = page("recab", "Write a system, the last word", "Formatting a slot asks for a typed word like every destructive step: here, the slot's letter.",
             recab(4, keys=[("B", "type it"), ("Enter", "write, once unlocked"), ("Esc", "cancel")]))
    p.heading(None, "Write over slot B?", "The previous system in it is gone, and with it the roll back.", SALMON)
    two_slots(p, ("running", [("untouched", MOSS), ("keeps running", MOSS)], "running"), ("idle", [("erased", SALMON), ("then LevitateOS 1.1", PAPER)], "formatted"))
    p.wordslots("B", "")

    p = page("recab", "Write a system, running", "The same figure and bar as the install's copy.", recab(4, keys=[("q", "wait for it")], right="writing slot B"))
    p.heading(None, "Writing slot B", "Slot A keeps running.")
    p.gap()
    p.figure("38%", x=p.x, row=p.y)
    top = p.y
    p.y = top - 1
    p.facts([("Files", "18,320 of 48,213"), ("Written", "1.2 of about 3.1 GiB"), ("Elapsed", "0:17"), ("Left", "about 0:28")], x=p.x + 26)
    p.y = top + 5
    p.bar(0.38)

    p = page("recab", "History", "What ran, down a line, newest first.", recab(6, history=" · 3"))
    p.heading(None, "This session", "recab keeps its own log in /var/lib/recab/audit.log.")
    p.timeline([("14:09:40", "done", "Slot B set for the next boot", "recab set-next B"), ("14:02:11", "done", "LevitateOS 1.1 written to slot B", "recab stage --from-media /dev/sr0"),
                ("14:01:48", "failed", "Roll back refused", "slot B had never been known-good")])

    p = page("recab", "No A/B here", "A system with one root has no slots: one block instead of two, and why.", recab(0, right="no slots"))
    p.heading("bang", "This system has one root", "It was installed as mutable: there are no slots to manage.", SALMON)
    p.slotbox(p.x, p.y, p.w, 5, "root", [("one system, changed in place", PAPER), ("updates cannot be tried first or rolled back", MOSS)], "running", "idle")
    p.note("Installed as A/B? Then /boot may not be mounted.")


# ====================================================================== the guide
GUIDE = ["Getting Started", "Installation", "Disk Preparation", "Base System", "Configuration", "Boot & Finish", "Post-Installation", "Troubleshooting"]


def guide(selected, cols=100, rows=28, distro="levitate", brand=True, rail=30):
    p = Deliberate(cols, rows, distro, rail=rail)
    p.band("Guide", GUIDE[selected], brand=brand)
    p.steps(len(GUIDE), selected, selected)
    p.menu([(None, [(name, "done" if i < selected else "none" if i != selected else "ready", "") for i, name in enumerate(GUIDE)]),
            (None, [("Package manager", "none", "▸"), ("Helpers", "none", "▸"), ("Architecture", "none", "▸"), ("Rec tooling", "none", "▸")])], selected)
    p.keys([("j/k", "scroll"), ("n/p", "command"), ("Enter", "copy"), ("Tab", "pages"), ("q", "quit")], f"page {selected + 1} of 34")
    return p


def numbered(p, n, title):
    p.put(p.y, p.x, str(n), p.accent, bold=True)
    p.put(p.y, p.x + len(str(n)) + 2, title, PAPER, bold=True)
    p.y += 1


def command(p, text, selected=False, width=None):
    width = width or p.w
    bg = p.accent if selected else INK
    p.fill(p.y, p.x, width, bg)
    p.put(p.y, p.x + 2, "$", NIGHT if selected else p.accent, bg, bold=True)
    p.put(p.y, p.x + 4, text, NIGHT if selected else p.highlight, bg, bold=selected)
    p.y += 1


def guide_pages():
    p = page("guide", "A guide page", "A page for reading: a narrow measure and numbered steps. The selected command is the bar; one key copies it.", guide(2))
    p.heading(3, "Disk Preparation", "Installation steps 1-5.")
    saved, p.w = p.w, 52
    p.text("disk-plan does these five steps for you and shows every command first. This page is the same steps by hand.", MOSS)
    p.gap()
    numbered(p, 2, "Identify Target Disk")
    p.text("List the disks and find the one to install on.", MOSS)
    p.gap()
    command(p, "lsblk -o NAME,SIZE,MODEL,TRAN", selected=True)
    p.gap()
    numbered(p, 3, "Plan the Partitions")
    p.table(["Partition", "Size", "Type", "Mount"], [["EFI", "1 GiB", "vfat", "/boot"], ["root A", "20 GiB", "ext4", "/"], ["root B", "20 GiB", "ext4", "none"]])
    p.w = saved

    p = page("guide", "A reference page", "A page of reference: a usage line and a table.", guide(3))
    p.heading(None, "recstrap", "Extract the base system to a target directory.")
    command(p, "recstrap [OPTIONS] <TARGET>")
    p.gap()
    p.table(["Option", "What it does"], [["--check", "run the twelve checks and stop"], ["--rootfs <PATH>", "copy this image, not the live one"], ["--json", "report progress as JSON lines"], ["--quiet", "print nothing but errors"]])
    p.gap()
    p.table(["Exit", "Meaning"], [["0", "copied"], ["E012", "the target is not empty"], ["E014", "not enough room on the target"]])


# ====================================================================== the install screen
def install_screen(distro="levitate"):
    """The hub and the guide side by side, as the live ISO opens: two panes of one
    screen, one band, the pane without the keyboard quieter."""
    cols, rows = 160, 30
    half = cols // 2
    left = hub(2, 2, half, rows - 1, distro, right="")
    launch(left)
    right = guide(3, half, rows - 1, distro, brand=False, rail=28)
    right.heading(4, "Base System", "Installation steps 6-9.")
    numbered(right, 6, "Verify Installation Media")
    right.text("The live media is already mounted. Check the system image is there:", MOSS)
    right.gap()
    command(right, "ls -la /run/live-rootfs.erofs")
    right.gap()
    numbered(right, 7, "Extract Base System")
    right.text("Use recstrap to extract the system image to your mounted root:", MOSS)
    right.gap()
    command(right, "recstrap /mnt")
    right.gap()
    numbered(right, 8, "Generate fstab")
    command(right, "recfstab --write /mnt")
    right.dim(0, half, 0.35)
    whole = Page(cols, rows, distro, rail=0)
    for r in range(rows - 1):
        whole.grid[r] = left.grid[r] + right.grid[r]
    whole.fill(rows - 1, 0, cols, INK)
    col = cols - 40
    for key, label in [("Ctrl-]", "other pane"), ("Ctrl-q", "quit")]:
        col = whole.put(rows - 1, col, f" {key} ", PAPER, RULE, bold=True)
        col = whole.put(rows - 1, col + 1, label, LICHEN, INK) + 3
    return whole


def install_screen_pages():
    page("install screen", "The install screen", "What the live ISO opens: the step you are at on the left, the guide open at the same step on the right, quieter because it does not have the keyboard.", install_screen())
    page("install screen", "The install screen, AcornOS", "The same screen on AcornOS.", install_screen("acorn"))


install_screen_pages()
PAGES.extend(_INSTALL)
system_setup_pages()
recab_pages()
guide_pages()

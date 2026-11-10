"""system-setup, recab, the guide, and the install screen that holds the hub and the guide.
Second pass: every page is laid out for the one thing it does."""
import pathlib
import re
from kit import *
from pages_install import PAGES as _INSTALL, hub, launch, STEP_KEYS, CONFIRM_KEYS

PAGES = []


def page(app, name, note, p):
    PAGES.append({"app": app, "name": name, "note": note, "page": p})
    return p


LIST_KEYS = [("↑/↓", "select"), ("PgUp/PgDn", "page"), ("Enter", "choose"), ("Bksp", "erase"), ("Esc", "cancel")]
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
    """The machine as a card with six lines: empty at the start, filled in as you answer,
    whole at the end. The overview, the review and the ending are this one card."""
    p.card(p.x, p.y, p.w, 9)
    p.fill(p.y, p.x, p.w, p.accent if filled == 6 else RULE)
    title = "studio" if filled >= 4 else "this machine"
    p.put(p.y, p.x + 2, title, NIGHT if filled == 6 else PAPER, bold=True)
    p.put(p.y, p.x + p.w - 16, "LevitateOS 1.0", NIGHT if filled == 6 else LICHEN)
    for i, (label, value) in enumerate(FIELDS):
        x = p.x + 2 + (i // 3) * (p.w // 2)
        row = p.y + 2 + (i % 3) * 2
        done = i < filled
        if numbered:
            p.put(row, x, f" {i + 1} ", PAPER, RULE, bold=True)
            x += 4
        else:
            p.put(row, x, "✓" if done else "●" if i == filled else "○", LEAF if done else p.accent if i == filled else GUTTER, INK)
            x += 2
        p.put(row, x, label, LICHEN, INK)
        p.put(row + 1, x, value if done else "·" * 12, PAPER if done else HAIR, INK, bold=done)
    p.y += 10


EUROPE = [("Amsterdam", "02:20", "+01:00", 52.4, 4.9), ("Athens", "03:20", "+02:00", 38.0, 23.7), ("Berlin", "02:20", "+01:00", 52.5, 13.4),
          ("Brussels", "02:20", "+01:00", 50.8, 4.3), ("Dublin", "01:20", "+00:00", 53.3, -6.3), ("Helsinki", "03:20", "+02:00", 60.2, 25.0),
          ("Lisbon", "01:20", "+00:00", 38.7, -9.1), ("London", "01:20", "+00:00", 51.5, -0.1), ("Madrid", "02:20", "+01:00", 40.4, -3.7),
          ("Oslo", "02:20", "+01:00", 59.9, 10.7), ("Paris", "02:20", "+01:00", 48.9, 2.3), ("Prague", "02:20", "+01:00", 50.1, 14.4)]
LIST_W = 34


def side(p, top, height):
    """The card beside a list: what the selected row would mean."""
    x, w = p.x + LIST_W + 2, p.w - LIST_W - 2
    p.card(x, top, w, height)
    return x, w


def zone_page(distro="levitate"):
    p = ss(2, 2, LIST_KEYS, distro=distro)
    p.heading(2, "Where is this machine?", "The clock of the new system.")
    top = p.y
    p.field("Search", "eu", "64 of 598", width=LIST_W)
    p.gap()
    p.rows_list([("Europe/", city, clock, offset) for city, clock, offset, _, _ in EUROPE], 0, LIST_W, match="eu")
    p.put(p.y, p.x + 1, "52 more below", GUTTER)
    x, w = side(p, top, 16)
    p.put(top + 1, x + 2, "Amsterdam", PAPER, INK, bold=True)
    p.put(top + 1, x + w - 7, " CET ", NIGHT, p.accent, bold=True)
    p.put(top + 2, x + 2, "Netherlands, Europe", LICHEN, INK)
    world(p, x + 2, top + 4, w - 4, 8, (52.4, 4.9), [(lat, lon) for *_, lat, lon in EUROPE])
    for i, (label, value, bold) in enumerate([("Local time", "02:20", True), ("From UTC", "+01:00", False)]):
        p.put(top + 13 + i, x + 2, label, LICHEN, INK)
        p.put(top + 13 + i, x + w - 2 - len(value), value, PAPER, INK, bold=bold)
    return p


def system_setup_pages():
    p = page("system-setup", "Overview", "The machine as a card with six empty lines. The whole app is filling it in.", ss(0, 1))
    p.heading("diamond", "Make this machine yours", "Six questions. Nothing changes until you have seen the commands.")
    machine_card(p, 0)
    p.button("Start with the keyboard")

    p = page("system-setup", "1 Keyboard", "Beside the list, the keyboard you would get, and a line to try it on: you see a wrong layout before it costs you a password.", ss(1, 1, LIST_KEYS))
    p.heading(1, "Which keyboard do you type on?", "It is loaded on this console as you move, so you can try it.")
    top = p.y
    p.field("Search", "de", "4 of 22", width=LIST_W)
    p.gap()
    p.rows_list([("", "de", "", ""), ("", "de-latin1", "", ""), ("", "de-latin1-nodeadkeys", "", ""), ("", "de_CH-latin1", "", "")], 0, LIST_W, match="de")
    x, w = side(p, top, 14)
    p.put(top + 1, x + 2, "de", PAPER, INK, bold=True)
    p.put(top + 1, x + w - 10, " QWERTZ ", NIGHT, p.accent, bold=True)
    p.put(top + 2, x + 2, "German", LICHEN, INK)
    for r, keys in enumerate(["q w e r t z u", "a s d f g h j", "y x c v b n m"]):
        col = x + 2 + r
        for key in keys.split():
            odd = key in "zy"
            p.put(top + 4 + r * 2, col, f" {key} ", NIGHT if odd else PAPER, p.highlight if odd else RAISED, bold=True)
            col += 4
    p.put(top + 10, x + 2, "TRY IT", GUTTER, INK, bold=True)
    p.fill(top + 11, x + 2, w - 4, RAISED)
    p.put(top + 11, x + 3, "zwölf yaks", PAPER, RAISED, bold=True)
    p.grid[top + 11][x + 13]["bg"] = p.accent
    p.put(top + 12, x + 2, "Tab moves here and back", GUTTER, INK)

    page("system-setup", "2 Time zone", "The place on a map and its clock right now: you check the time against your watch, not a name against a list.", zone_page())

    p = page("system-setup", "3 Language", "Beside the list, what the choice changes: today's date, the time, a number, the first day of the week.", ss(3, 3, LIST_KEYS))
    p.heading(3, "Which language?", "The system's language and how it writes dates and numbers.")
    top = p.y
    p.field("Search", "en", "2 of 16", width=LIST_W)
    p.gap()
    p.rows_list([("", "en_GB.UTF-8", "", ""), ("", "en_US.UTF-8", "", "")], 1, LIST_W, match="en")
    x, w = side(p, top, 12)
    p.put(top + 1, x + 2, "English", PAPER, INK, bold=True)
    p.put(top + 1, x + w - 6, " US ", NIGHT, p.accent, bold=True)
    p.put(top + 2, x + 2, "as the system will write", LICHEN, INK)
    for i, (label, value) in enumerate([("Date", "Tuesday, November 10, 2026"), ("Time", "2:20 AM"), ("Number", "1,234,567.89"), ("Week starts", "Sunday")]):
        p.put(top + 4 + i * 2, x + 2, label.upper(), GUTTER, INK, bold=True)
        p.put(top + 5 + i * 2, x + 2, value, PAPER, INK)

    p = page("system-setup", "4 Hostname", "The name where it will appear, as you type it, and the rules it must keep, each with its mark.", ss(4, 4, TEXT_KEYS))
    p.heading(4, "What is this machine called?", "Its name on the prompt and on the network.")
    p.field("Hostname", "studio", width=40)
    p.gap()
    p.card(p.x, p.y, p.w, 4)
    p.put(p.y, p.x + 2, "WHERE YOU WILL SEE IT", GUTTER, INK, bold=True)
    p.runs(p.y + 1, p.x + 2, [("root@", MOSS), ("studio", p.accent, True), (" ~ # ", MOSS)], INK)
    p.runs(p.y + 2, p.x + 2, [("studio", p.accent, True), (" login:", MOSS)], INK)
    p.runs(p.y + 1, p.x + 34, [("on the network  ", LICHEN), ("studio", p.accent, True), (".local", MOSS)], INK)
    p.y += 5
    p.checks([("The name", [("done", "letters, digits and hyphens only"), ("done", "no hyphen first or last"), ("done", "64 characters at most")])])

    p = page("system-setup", "4 Hostname, refused", "The character that breaks the rule is marked in the field, and the rule it breaks takes the cross.", ss(4, 4, TEXT_KEYS))
    p.heading(4, "What is this machine called?", "Its name on the prompt and on the network.")
    p.field("Hostname", "studio_pc", width=40)
    p.put(p.y - 1, p.x + 17, "_", NIGHT, SALMON, bold=True)
    p.gap()
    p.panel("danger", "'studio_pc' is not a valid hostname", ["An underscore is not allowed. Try studio-pc."])
    p.checks([("The name", [("failed", "letters, digits and hyphens only"), ("done", "no hyphen first or last"), ("done", "64 characters at most")])])

    p = page("system-setup", "5 Root password", "Two entries with their state beside them, a strength line, the keyboard you are typing on, and what the choice means.", ss(5, 5, TEXT_KEYS))
    p.heading(5, "A password for root?", "The system's own administrator account.")
    top = p.y
    p.field("Password ", "••••••••••", width=LIST_W, cursor=False)
    p.put(p.y - 1, p.x + LIST_W - 3, "✓", LEAF, RAISED)
    p.meter(0.75, "10 characters", width=LIST_W - 16)
    p.gap()
    p.field("Once more", "••••••", width=LIST_W)
    p.put(p.y, p.x + 1, "matches so far", LEAF)
    p.y += 2
    p.runs(p.y, p.x, [("typed on the ", LICHEN), (" de ", NIGHT, True)])
    p.put(p.y, p.x + 13, " de ", NIGHT, p.accent, bold=True)
    p.put(p.y, p.x + 18, "keyboard", LICHEN)
    x, w = side(p, top, 11)
    p.put(top + 1, x + 2, "WITH A PASSWORD", GUTTER, INK, bold=True)
    for i, line in enumerate(wrap("You can log in as root on the console, and repair the system when nothing else works.", w - 4)):
        p.put(top + 2 + i, x + 2, line, MOSS, INK)
    p.put(top + 6, x + 2, "LEFT EMPTY", GUTTER, INK, bold=True)
    for i, line in enumerate(wrap("Root cannot log in. Only your own user administers, with sudo.", w - 4)):
        p.put(top + 7 + i, x + 2, line, MOSS, INK)

    p = page("system-setup", "6 User", "The account as it will exist, built up as you type: its name, what it may do, where it lives.", ss(6, 6, TEXT_KEYS))
    p.heading(6, "Who uses this machine?", "Your own account. Leave the name empty to create none.")
    top = p.y
    p.field("User name", "ann", width=LIST_W)
    p.gap()
    p.checks([("The name", [("done", "starts with a lowercase letter"), ("done", "lowercase, digits, _ or -"), ("done", "not taken")])], width=LIST_W)
    x, w = side(p, top, 10)
    for r in range(3):
        p.fill(top + 1 + r, x + 2, 7, p.accent)
    p.put(top + 2, x + 5, "A", NIGHT, p.accent, bold=True)
    p.put(top + 1, x + 11, "ann", PAPER, INK, bold=True)
    p.put(top + 2, x + 11, "administrator", p.accent, INK)
    p.put(top + 3, x + 11, "may use sudo", LICHEN, INK)
    for i, (label, value) in enumerate([("Home", "/home/ann"), ("Shell", "/bin/bash"), ("Groups", "ann, wheel")]):
        p.put(top + 5 + i, x + 2, label, LICHEN, INK)
        p.put(top + 5 + i, x + 10, value, PAPER, INK)
    p.runs(top + 9, x + 2, [("studio login: ", GUTTER), ("ann", PAPER, True)], INK)
    p.y = top + 11
    p.hint([("Enter",), " accepts the name and asks for its password."])

    p = page("system-setup", "7 Apply, reviewing", "The card, filled. Each line carries its number: one key goes back to change it. Then one bar.", ss(7, 7, keys=[("1-6", "change an answer"), ("space", "the commands"), ("Enter", "apply"), ("q", "back")]))
    p.heading(7, "Is this right?", "Nothing has changed yet. recsetup will run 6 commands, in order.")
    machine_card(p, 6, numbered=True)
    p.button("Apply 6 steps to /mnt")

    p = page("system-setup", "7 Apply, the commands", "The same page turned over: the exact commands, as a run sheet.", ss(7, 7, keys=[("space", "the answers"), ("Enter", "apply"), ("q", "back")]))
    p.heading(7, "Is this right?", "Nothing has changed yet. recsetup will run 6 commands, in order.")
    for n, (what, command) in enumerate([("Time zone", "recipe-system-setup timezone Europe/Amsterdam"), ("Language", "recipe-system-setup locale en_US.UTF-8"), ("Keyboard", "recipe-system-setup keymap de"),
                                         ("Hostname", "recipe-system-setup hostname studio"), ("User", "recipe-system-setup user ann /bin/bash wheel"), ("Passwords", "chpasswd   (root, ann: read from stdin)")]):
        p.put(p.y, p.x, what, MOSS, bold=True)
        p.put(p.y, p.x + 10, "┃", HAIR)
        p.put(p.y, p.x + 12, str(n + 1).rjust(2), GUTTER)
        p.put(p.y, p.x + 16, "○", GUTTER)
        p.put(p.y, p.x + 19, command, PAPER)
        p.y += 1
    p.gap()
    p.text("Each runs inside the new system, through recchroot /mnt.", LICHEN)
    p.gap()
    p.button("Apply 6 steps to /mnt")

    p = page("system-setup", "7 Apply, asking", "The yes-or-no panel, on top as everywhere.", ss(7, 7, CONFIRM_KEYS))
    p.heading(7, "Is this right?", "Nothing has changed yet. recsetup will run 6 commands, in order.")
    p.panel("warning", "Apply these 6 steps to /mnt?", ["This writes to the new system."], keys=[("y", "apply"), ("n", "not yet")])
    machine_card(p, 6, numbered=True)

    p = page("system-setup", "7 Apply, stopped", "The command that failed is marked on the run sheet with what it printed, and the bar offers the retry.", ss(7, 7, right="stopped", states={7: "failed"}))
    p.heading("bang", "The configuration stopped", "At command 5 of 6. The command after it did not run.", SALMON)
    for n, (what, command, state) in enumerate([("Time zone", "timezone Europe/Amsterdam", "done"), ("Language", "locale en_US.UTF-8", "done"), ("Keyboard", "keymap de", "done"),
                                                ("Hostname", "hostname studio", "done"), ("User", "user ann /bin/bash wheel", "failed"), ("Passwords", "chpasswd", "todo")]):
        here = state == "failed"
        p.put(p.y, p.x, what, MOSS if state != "todo" else GUTTER, bold=True)
        p.put(p.y, p.x + 10, "┃", HAIR)
        if here:
            p.fill(p.y, p.x + 12, p.w - 12, RAISED)
        mark, colour = MARKS[state]
        p.put(p.y, p.x + 13, mark, colour, RAISED if here else None, bold=here)
        p.put(p.y, p.x + 16, command, PAPER if state != "todo" else LICHEN, RAISED if here else None, bold=here)
        p.y += 1
    p.gap()
    p.panel("danger", "useradd: cannot lock /etc/passwd; try again later", ["Exit status 9. The four commands before it are done and are safe to run again."])
    p.button("Apply again")

    p = page("system-setup", "Done", "The card once more, now true, and one bar on.", ss(8, 9))
    p.heading("check", "This machine is yours", "It knows its keyboard, clock, language and name, and who may log in.", LEAF)
    machine_card(p, 6)
    p.button("Continue: make it boot")

    page("system-setup", "2 Time zone, AcornOS", "The same page on AcornOS: its band and its accent, nothing else differs.", zone_page("acorn"))


# ====================================================================== recab
def recab(selected, running="A", trial=False, keys=None, right=None, history=""):
    p = Deliberate()
    place = ["Overview", "Try slot B", "Commit", "Roll back", "Write a system", "Reboot", "History"][selected]
    p.band("Updates", place)
    seg = (p.cols - 2 - 1) // 2
    for i, (name, state) in enumerate([("Slot A · running", "running"), ("Slot B · next boot, once" if trial else "Slot B · previous system", "next" if trial else "idle")]):
        colour = {"running": p.accent, "next": p.highlight, "idle": RULE}[state]
        p.fill(2, 1 + i * (seg + 1), seg, colour)
        p.put(2, 3 + i * (seg + 1), name, NIGHT if state != "idle" else LICHEN, bold=state != "idle")
    p.menu([
        (None, [("Overview", "none", "o")]),
        ("Changes", [("Try slot B", "ready", "t"), ("Commit slot A", "todo", "c"), ("Roll back to B", "ready", "x"), ("Write to slot B", "ready", "s"), ("Reboot", "ready", "R")]),
        ("Session", [("History" + history, "none", "H")]),
    ], selected)
    p.keys(keys or [("j/k", "select"), ("Enter", "run it"), ("r", "read again"), ("q", "quit")], right or "slot A")
    return p


def two_slots(p, a, b, h=6):
    """The two slots as blocks, side by side: the picture every recab page is about."""
    w = (p.w - 3) // 2
    p.slotbox(p.x, p.y, w, h, "SLOT A", a[1], a[2], a[0])
    p.slotbox(p.x + w + 3, p.y, w, h, "SLOT B", b[1], b[2], b[0])
    p.y += h + 1


def route(p, steps, x=None):
    """A boot as a line of stops: (text, colour)."""
    col = p.x if x is None else x
    for i, (text, colour) in enumerate(steps):
        col = p.put(p.y, col, text, colour, bold=i == len(steps) - 1)
        if i < len(steps) - 1:
            col = p.put(p.y, col + 1, "─▶", GUTTER) + 1
    p.y += 1


def recab_pages():
    p = page("recab", "Overview", "The two slots as two blocks, and the next boot as a line you can follow with a finger.", recab(0))
    p.heading("slots", "Two systems, one running", "An update is written to the other slot and tried once before it is kept.")
    two_slots(p, ("running", [("LevitateOS 1.0", PAPER), ("known-good", LEAF), ("boots by default", MOSS)], "running"),
              ("idle", [("the previous system", PAPER), ("you can roll back to it", MOSS), ("last trial: failed, 5 Nov", SALMON)], ""))
    p.put(p.y, p.x, "THE NEXT BOOT", GUTTER, bold=True)
    p.y += 1
    route(p, [("power on", LICHEN), ("systemd-boot", LICHEN), ("slot A", p.accent)])
    route(p, [("if you roll back", LICHEN), ("slot B", PAPER)])
    p.gap()
    p.hint([("s",), " writes a new system to slot B.   ", ("x",), " rolls back."])

    p = page("recab", "Overview, a trial pending", "A pending trial lights slot B in the second accent everywhere at once: the slot line, its block, and the boot line, which now forks.", recab(0, trial=True))
    p.heading("slots", "Slot B boots next, once", "A trial: kept if it starts healthy, undone if it does not.")
    two_slots(p, ("running", [("LevitateOS 1.0", PAPER), ("known-good", LEAF), ("where a failed trial returns", MOSS)], "running"),
              ("next", [("LevitateOS 1.1", PAPER), ("written today, 14:02", MOSS), ("never booted", MOSS)], "next boot"))
    p.put(p.y, p.x, "THE NEXT BOOT", GUTTER, bold=True)
    p.y += 1
    route(p, [("power on", LICHEN), ("slot B, once", p.highlight)])
    p.put(p.y, p.x + 12, "├─ starts healthy ─▶", GUTTER)
    p.put(p.y, p.x + 33, "B is kept and becomes the default", LEAF)
    p.put(p.y + 1, p.x + 12, "└─ does not ─▶", GUTTER)
    p.put(p.y + 1, p.x + 33, "back to slot A, nothing lost", PAPER)
    p.y += 3
    p.button("Reboot into the trial", key="R")

    p = page("recab", "Try slot B", "Before a change: the fork it sets up, drawn, and its one command.", recab(1))
    p.heading("slots", "Try slot B on the next boot", "Boot it once and see. Slot A stays as it is.")
    p.put(p.y, p.x, "WHAT WILL HAPPEN", GUTTER, bold=True)
    p.y += 1
    p.card(p.x, p.y, p.w, 5)
    p.put(p.y + 1, p.x + 2, "reboot ─▶", LICHEN, INK)
    p.put(p.y + 1, p.x + 12, " slot B, once ", NIGHT, p.highlight, bold=True)
    p.put(p.y + 1, p.x + 27, "─┬─ starts healthy ─▶", GUTTER, INK)
    p.put(p.y + 1, p.x + 49, "B is kept", LEAF, INK, bold=True)
    p.put(p.y + 3, p.x + 27, " └─ does not ──────▶", GUTTER, INK)
    p.put(p.y + 2, p.x + 28, "│", GUTTER, INK)
    p.put(p.y + 3, p.x + 49, "back to slot A", PAPER, INK, bold=True)
    p.y += 6
    p.commands(["recab set-next B"])
    p.gap()
    p.button("Set slot B for the next boot")

    p = page("recab", "Try slot B, asking", "The yes-or-no panel, with the picture still under it.", recab(1, keys=CONFIRM_KEYS))
    p.heading("slots", "Try slot B on the next boot", "Boot it once and see. Slot A stays as it is.")
    p.panel("warning", "Run recab set-next B?", ["Nothing happens until you reboot."], keys=[("y", "run it"), ("n", "cancel")])
    two_slots(p, ("running", [("stays the default", MOSS), ("stays known-good", MOSS)], "running"), ("next", [("boots once, next", PAPER), ("kept only if healthy", MOSS)], "after"), h=5)

    p = page("recab", "Try slot B, done", "After a change: the slots as they were, an arrow, the slots as they are.", recab(1, trial=True, history=" · 1"))
    p.heading("check", "Slot B boots next, once", "Set in 0.1 s. Reboot to start the trial.", LEAF)
    w = (p.w - 9) // 2
    for i, (label, a, b) in enumerate([("BEFORE", ("running", "default"), ("idle", "previous")), ("AFTER", ("running", "default"), ("next", "next boot, once"))]):
        x = p.x + i * (w + 9)
        p.put(p.y, x, label, GUTTER, bold=True)
        for j, (name, (state, text)) in enumerate([("A", a), ("B", b)]):
            colour = {"running": p.accent, "next": p.highlight, "idle": RULE}[state]
            bw = (w - 1) // 2
            bx = x + j * (bw + 1)
            p.card(bx, p.y + 1, bw, 3, colour)
            p.put(p.y + 1, bx + 1, f"SLOT {name}", NIGHT if state != "idle" else PAPER, bold=True)
            p.put(p.y + 2, bx + 1, text, NIGHT if state != "idle" else LICHEN)
    p.put(p.y + 2, p.x + w + 3, "─▶", p.accent, bold=True)
    p.y += 6
    p.button("Reboot into the trial", key="R")

    p = page("recab", "Roll back, refused", "A change recab would refuse says so before you press anything, and shows why on the slots themselves.", recab(3))
    p.heading("bang", "Nothing to roll back to", "recab would refuse this now.", SALMON)
    two_slots(p, ("running", [("committed", LEAF), ("the only known-good system", MOSS)], "running"), ("idle", [("written 14:02, never booted", PAPER), ("not known-good", SALMON)], ""), h=5)
    p.panel("danger", "Slot B has never been known-good", ["A roll back goes to a system that has already run well. Try slot B first; once it is committed, slot A becomes the one to roll back to."])
    p.hint([("t",), " tries slot B."])

    p = page("recab", "Write a system, choosing", "Where the new system comes from is a choice from what is plugged in, not three paths to type. What it will take is ticked beside it.", recab(4, keys=[("↑/↓", "select"), ("Enter", "choose"), ("Esc", "cancel")]))
    p.heading("slots", "Write a new system to slot B", "Your accounts, hostname, network and services are carried over.")
    p.put(p.y, p.x, "FROM", GUTTER, bold=True)
    p.y += 1
    for i, (title, where, parts, chosen) in enumerate([("LevitateOS 1.1 release media", "/dev/sr0", "system image ✓   kernel ✓   initramfs ✓", True),
                                                      ("LevitateOS 1.0 release media", "/dev/sdb", "the version slot A runs", False), ("An image file…", "", "choose the image, kernel and initramfs by hand", False)]):
        bg = RAISED if chosen else None
        if chosen:
            p.card(p.x, p.y, p.w, 2, RAISED)
            p.put(p.y, p.x, "█", p.accent)
            p.put(p.y + 1, p.x, "█", p.accent)
        p.put(p.y, p.x + 2, ("● " if chosen else "○ ") + title, p.accent if chosen else PAPER, bg, bold=chosen)
        p.put(p.y, p.x + p.w - 2 - len(where), where, LICHEN, bg)
        p.put(p.y + 1, p.x + 4, parts, LEAF if chosen else LICHEN, bg)
        p.y += 3
    p.put(p.y, p.x, "TO", GUTTER, bold=True)
    p.y += 1
    route(p, [("slot B", PAPER), ("formatted", SALMON), ("LevitateOS 1.1", p.accent)])

    p = page("recab", "Write a system, the last word", "Formatting a slot is destructive, so it asks for a typed word like every destructive step: here, the slot's letter.",
             recab(4, keys=[("B", "type it"), ("Enter", "write, once unlocked"), ("Esc", "cancel")]))
    p.heading("slots", "Write over slot B?", "The previous system in it is gone, and with it the roll back.", SALMON)
    two_slots(p, ("running", [("untouched", LEAF), ("keeps running", MOSS)], "running"), ("idle", [("the previous system", PAPER), ("erased, then LevitateOS 1.1", SALMON)], "formatted"), h=5)
    p.put(p.y, p.x, "TYPE THE SLOT'S LETTER", GUTTER, bold=True)
    p.y += 1
    p.wordslots("B", "")

    p = page("recab", "Write a system, running", "Writing is the long job here: the same figure and bar as the install's copy.", recab(4, keys=[("q", "wait for it")], right="writing slot B"))
    p.heading("slots", "Writing slot B", "From the release media. Slot A keeps running.")
    p.figure("38%", x=p.x, row=p.y)
    top = p.y
    p.facts([("Files", "18,320 of 48,213"), ("Written", "1.2 of about 3.1 GiB"), ("Elapsed", "0:17"), ("Left", "about 0:28")], x=p.x + 24)
    p.y = top + 5
    p.bar(0.38)
    p.gap()
    p.minipath([("Format", "done"), ("Copy", "running"), ("Carry over accounts", "todo"), ("Kernel", "todo")])

    p = page("recab", "History", "What ran, as a line down the page, newest first.", recab(6, history=" · 3"))
    p.heading("slots", "This session", "recab keeps its own log in /var/lib/recab/audit.log.")
    p.timeline([("14:09:40", "done", "Slot B set for the next boot", "recab set-next B · 0.1 s"), ("14:02:11", "done", "LevitateOS 1.1 written to slot B", "recab stage --from-media /dev/sr0 · 0:45"),
                ("14:01:48", "failed", "Roll back refused", "slot B had never been known-good")])

    p = page("recab", "No A/B here", "A system with one root has no slots: the page says which kind of system this is, not just that reading failed.", recab(0, right="no slots"))
    p.heading("bang", "This system has one root", "It was installed as mutable: there are no slots to manage.", SALMON)
    p.card(p.x, p.y, p.w, 5, RAISED)
    p.fill(p.y, p.x, p.w, RULE)
    p.put(p.y, p.x + 2, "ROOT", PAPER, bold=True)
    p.put(p.y + 2, p.x + 2, "one system, changed in place", PAPER, RAISED)
    p.put(p.y + 3, p.x + 2, "updates cannot be tried first or rolled back", LICHEN, RAISED)
    p.y += 6
    p.panel("info", "recab found no slot entries under /boot/loader/entries", ["If this machine was installed as A/B, the EFI partition may not be mounted at /boot."])
    p.hint([("r",), " reads it again."])


# ====================================================================== the guide
GUIDE = ["Getting Started", "Installation", "Disk Preparation", "Base System", "Configuration", "Boot & Finish", "Post-Installation", "Troubleshooting"]


def guide(selected, cols=100, rows=28, distro="levitate", brand=True, rail=30):
    p = Deliberate(cols, rows, distro, rail=rail)
    p.band("Guide", GUIDE[selected], brand=brand)
    p.steps(len(GUIDE), selected, selected)
    p.menu([("Getting started", [(name, "done" if i < selected else "none" if i != selected else "ready", "") for i, name in enumerate(GUIDE)]),
            (None, [("Package manager", "none", "▸"), ("Helpers", "none", "▸"), ("Architecture", "none", "▸"), ("Rec tooling", "none", "▸")])], selected)
    p.keys([("j/k", "scroll"), ("n/p", "command"), ("Enter", "copy"), ("Tab", "pages"), ("q", "quit")], f"page {selected + 1} of 34")
    return p


def numbered(p, n, title):
    p.put(p.y, p.x, f" {n} ", NIGHT, p.accent, bold=True)
    p.put(p.y, p.x + len(str(n)) + 3, title, PAPER, bold=True)
    p.y += 1


def command(p, text, selected=False, width=None):
    width = width or p.w
    bg = p.accent if selected else INK
    p.fill(p.y, p.x, width, bg)
    p.put(p.y, p.x + 2, "$", NIGHT if selected else p.accent, bg, bold=True)
    p.put(p.y, p.x + 4, text, NIGHT if selected else p.highlight, bg, bold=selected)
    if selected:
        p.put(p.y, p.x + width - 8, " Enter ", p.accent, NIGHT, bold=True)
    p.y += 1


def guide_pages():
    p = page("guide", "A guide page", "A page for reading: a narrow measure, numbered steps, and beside it where you are on the page. The step the install is at is marked.", guide(2))
    measure = p.w - 22
    p.heading(3, "Disk Preparation", "Installation steps 1-5.")
    saved = p.w
    p.w = measure
    p.panel("info", "These steps in one tool", ["disk-plan does all five, and shows every command first."])
    numbered(p, 2, "Identify Target Disk")
    p.text("List the disks and find the one to install on.", MOSS)
    p.gap()
    command(p, "lsblk -o NAME,SIZE,MODEL,TRAN", selected=True)
    p.gap()
    numbered(p, 3, "Plan the Partitions")
    p.table(["Partition", "Size", "Type", "Mount"], [["EFI", "1 GiB", "vfat", "/boot"], ["root A", "20 GiB", "ext4", "/"], ["root B", "20 GiB", "ext4", "none"]])
    p.w = saved
    x = p.x + measure + 3
    top = p.HEADER + 1
    p.card(x, top, 18, 9)
    p.put(top, x + 2, "ON THIS PAGE", GUTTER, INK, bold=True)
    for i, (n, title, state) in enumerate([(1, "Boot the ISO", "done"), (2, "Find the disk", "here"), (3, "Plan it", "todo"), (4, "Format, mount", "todo"), (5, "Check mounts", "todo")]):
        colour = p.accent if state == "here" else LEAF if state == "done" else GUTTER
        p.put(top + 2 + i, x + 2, "✓" if state == "done" else str(n), colour, INK, bold=state == "here")
        p.put(top + 2 + i, x + 4, title[:13], PAPER if state == "here" else LICHEN, INK, bold=state == "here")
    p.scrollbar(0.15, 0.4)

    p = page("guide", "A reference page", "A page of reference is mostly a table and an example; the same furniture, wider.", guide(3))
    p.heading("page", "recstrap", "Extract the base system to a target directory.")
    numbered(p, "$", "Usage")
    command(p, "recstrap [OPTIONS] <TARGET>")
    p.gap()
    numbered(p, "#", "Options")
    p.table(["Option", "What it does"], [["--check", "run the twelve checks and stop"], ["--rootfs <PATH>", "copy this image, not the live one"], ["--json", "report progress as JSON lines"], ["--quiet", "print nothing but errors"]],
            colours=[None, None, None, None])
    p.gap()
    numbered(p, "!", "Exit codes")
    p.table(["Code", "Meaning"], [["0", "copied"], ["E012", "the target is not empty"], ["E014", "not enough room on the target"]], colours=[LEAF, SALMON, SALMON])
    p.scrollbar(0.0, 0.5)


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
    whole.put(rows - 1, 2, "The install steps have the keyboard", GUTTER, INK)
    col = cols - 40
    for key, label in [("Ctrl-]", "other pane"), ("Ctrl-q", "quit")]:
        col = whole.put(rows - 1, col, f" {key} ", PAPER, RULE, bold=True)
        col = whole.put(rows - 1, col + 1, label, LICHEN, INK) + 3
    return whole


def install_screen_pages():
    page("install screen", "The install screen", "What the live ISO opens: the step you are at as a launch pad on the left, the guide open at the same step on the right, quieter because it does not have the keyboard.", install_screen())
    page("install screen", "The install screen, AcornOS", "The same screen on AcornOS.", install_screen("acorn"))


install_screen_pages()
PAGES.extend(_INSTALL)
system_setup_pages()
recab_pages()
guide_pages()

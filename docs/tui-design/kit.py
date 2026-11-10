"""The design system as code: every mock-up page is built from these parts and nothing
else, so the pages cannot drift apart. A page is a grid of terminal cells (character,
colours, bold); only what a terminal can show is used."""
import re

# ---- Colours: the website's dark surfaces and the two distros' accents.
NIGHT, INK, RAISED = "#0d1711", "#14221a", "#1d2d24"
HAIR, RULE, GUTTER = "#2a3b31", "#3a4b40", "#6e7f73"
LICHEN, MOSS, PAPER = "#9aa89d", "#bac5b7", "#f2eee4"
SAGE, SAND, LEAF, SALMON = "#c7d9ba", "#e4cba6", "#a9cf9a", "#f0a497"
BRANDS = {
    "levitate": {"name": "LevitateOS", "accent": SAGE, "highlight": SAND, "band": "#214438"},
    "acorn": {"name": "AcornOS", "accent": SAND, "highlight": SAGE, "band": "#644532"},
}
TONES = {"success": LEAF, "warning": SAND, "danger": SALMON, "info": MOSS}
TONE_MARK = {"success": "✓", "warning": "!", "danger": "!", "info": "i"}

# ---- Marks: one vocabulary for every list of steps or actions.
MARKS = {
    "done": ("✓", LEAF), "here": ("◉", None), "ready": ("●", None), "todo": ("○", GUTTER),
    "running": ("◐", SAND), "failed": ("✗", SALMON), "none": (" ", GUTTER),
}

# ---- Big glyphs: five by six pixels, drawn two pixel rows to a cell.
PIXELS = {
    "0": [".###.", "#...#", "#...#", "#...#", "#...#", ".###."],
    "1": ["..#..", ".##..", "..#..", "..#..", "..#..", ".###."],
    "2": [".###.", "#...#", "...#.", "..#..", ".#...", "#####"],
    "3": ["####.", "....#", ".###.", "....#", "....#", "####."],
    "4": ["...#.", "..##.", ".#.#.", "#####", "...#.", "...#."],
    "5": ["#####", "#....", "####.", "....#", "....#", "####."],
    "6": [".###.", "#....", "####.", "#...#", "#...#", ".###."],
    "7": ["#####", "....#", "...#.", "..#..", "..#..", "..#.."],
    "8": [".###.", "#...#", ".###.", "#...#", "#...#", ".###."],
    "9": [".###.", "#...#", "#...#", ".####", "....#", ".###."],
    "check": ["....#", "...##", "#.##.", "###..", ".#...", "....."],
    "bang": ["..#..", "..#..", "..#..", "..#..", ".....", "..#.."],
    }


def big(glyph):
    """The three text rows of a glyph or a number."""
    parts = [PIXELS[digit] for digit in str(glyph)] if isinstance(glyph, int) else [PIXELS[glyph]]
    rows = []
    for r in range(3):
        cells = []
        for px in parts:
            line = ""
            for c in range(5):
                top, bottom = px[r * 2][c] == "#", px[r * 2 + 1][c] == "#"
                line += "█" if top and bottom else "▀" if top else "▄" if bottom else " "
            cells.append(line)
        rows.append(" ".join(cells))
    return rows


def mix(a, b, t):
    pa, pb = [int(a[i:i + 2], 16) for i in (1, 3, 5)], [int(b[i:i + 2], 16) for i in (1, 3, 5)]
    return "#%02x%02x%02x" % tuple(round(x + (y - x) * max(0, min(1, t))) for x, y in zip(pa, pb))


def wrap(text, width):
    lines, line = [], ""
    for word in text.split(" "):
        if line and len(line) + 1 + len(word) > width:
            lines.append(line)
            line = word
        else:
            line = f"{line} {word}" if line else word
    return lines + ([line] if line else [])


class Page:
    """One screen. The frame is the same on every page: band, step line, rail beside the
    content, keys. Content is laid out top to bottom from `self.y`."""

    RAIL = 28
    HEADER = 4  # band, space, step line, space

    def __init__(self, cols=100, rows=28, distro="levitate", rail=None):
        self.cols, self.rows = cols, rows
        self.brand = BRANDS[distro]
        self.accent, self.highlight = self.brand["accent"], self.brand["highlight"]
        self.rail = self.RAIL if rail is None else rail
        self.grid = [[{"ch": " ", "fg": PAPER, "bg": NIGHT, "bold": False, "dim": False, "inv": False}
                      for _ in range(cols)] for _ in range(rows)]
        for r in range(self.HEADER):
            self.fill(r, 0, cols, INK)
        for r in range(self.HEADER, rows - 2):
            self.fill(r, 0, self.rail, INK)
        for r in (rows - 2, rows - 1):
            self.fill(r, 0, cols, INK)
        self.x = self.rail + 3 if self.rail else 2
        self.w = cols - self.x - 2
        self.y = self.HEADER + 1
        self.bottom = rows - 4

    # -- cells
    def put(self, row, col, text, fg=PAPER, bg=None, bold=False):
        if not (0 <= row < self.rows):
            return col + len(text)
        for i, ch in enumerate(text):
            if 0 <= col + i < self.cols:
                cell = self.grid[row][col + i]
                cell.update(ch=ch, fg=fg, bold=bold)
                if bg:
                    cell["bg"] = bg
        return col + len(text)

    def fill(self, row, col, width, bg):
        if 0 <= row < self.rows:
            for c in range(max(0, col), min(self.cols, col + width)):
                self.grid[row][c]["bg"] = bg

    def runs(self, row, col, parts, bg=None):
        """Text in several styles: (text, colour[, bold]) parts."""
        for part in parts:
            text, fg, bold = (part + (False,))[:3] if len(part) < 3 else part
            col = self.put(row, col, text, fg, bg, bold)
        return col

    # -- frame
    def band(self, app, place="", brand=True):
        for c in range(self.cols):
            self.grid[0][c]["bg"] = mix(self.brand["band"], INK, c / (self.cols * 0.7)) if brand else INK
        col = 1
        if brand:
            col = self.put(0, col, "◆", self.accent, bold=True)
            col = self.put(0, col + 1, self.brand["name"], PAPER, bold=True)
        right = f"{app}{'   ' + place if place else ''} "
        at = self.cols - len(right)
        end = self.put(0, at, app, PAPER, bold=True)
        if place:
            self.put(0, end, f"   {place}", LICHEN)
        return self

    def steps(self, total, done=0, current=None, labels=None, lit=None):
        """The step line: a segment a step, a full row thick. With `labels` the segments
        are named (recab's two slots); `lit` marks named segments that are on."""
        seg = (self.cols - 2 - (total - 1)) // total
        for i in range(total):
            start = 1 + i * (seg + 1)
            if labels:
                on = lit and lit[i]
                colour = self.accent if on else RULE
                self.fill(2, start, seg, colour)
                self.put(2, start + 2, labels[i], NIGHT if on else LICHEN, bold=bool(on))
            else:
                colour = LEAF if i < done else self.accent if i == current else RULE
                self.put(2, start, "█" * seg, colour)
        return self

    def path(self, steps, selected):
        """The rail as a path: (label, state, value) steps joined by a line."""
        row = self.HEADER + 1
        joined = (self.rows - 2 - self.HEADER - 2) >= len(steps) * 2 - 1
        for i, (label, state, value) in enumerate(steps):
            here = i == selected
            bg = RAISED if here else None
            if here:
                self.fill(row, 0, self.rail, RAISED)
                self.put(row, 0, "▌", self.accent)
            mark, colour = MARKS["here" if here and state not in ("done", "failed", "running") else state]
            self.put(row, 3, mark, colour or self.accent, bg, bold=here)
            self.put(row, 6, label[: self.rail - 8], PAPER if here else LICHEN if state == "todo" else MOSS, bg, bold=here)
            room = self.rail - 8 - len(label) - 2
            if value and room >= 3:
                value = value if len(value) <= room else value[: room - 1] + "…"
                self.put(row, self.rail - 2 - len(value), value, LICHEN, bg)
            if joined and i < len(steps) - 1:
                self.put(row + 1, 3, "│", LEAF if state == "done" else HAIR)
            row += 2 if joined else 1
        return self

    def menu(self, groups, selected):
        """The rail as a list: groups of (label, state, right) rows under a small label.
        The same rows as the path, without the joining line."""
        row = self.HEADER + 1
        index = 0
        for title, items in groups:
            if title:
                self.put(row, 3, title.upper(), GUTTER, bold=True)
                row += 1
            for label, state, right in items:
                here = index == selected
                bg = RAISED if here else None
                if here:
                    self.fill(row, 0, self.rail, RAISED)
                    self.put(row, 0, "▌", self.accent)
                mark, colour = MARKS[state]
                self.put(row, 3, mark, colour or self.accent, bg, bold=here)
                self.put(row, 6, label[: self.rail - 10], PAPER if here else LICHEN if state == "todo" else MOSS, bg, bold=here)
                if right:
                    self.put(row, self.rail - 2 - len(right), right, GUTTER, bg)
                row += 1
                index += 1
            row += 1
        return self

    def keys(self, pairs, right="", right_colour=GUTTER):
        """The footer: each key a keycap, then what it does; context at the line's end."""
        row, col = self.rows - 2, 2
        for key, label in pairs:
            col = self.put(row, col, f" {key} ", PAPER, RULE, bold=True)
            col = self.put(row, col + 1, label, LICHEN) + 3
        if right:
            self.put(row, self.cols - 2 - len(right), right, right_colour)
        return self

    # -- content blocks, top to bottom
    def gap(self, n=1):
        self.y += n
        return self

    def heading(self, glyph, title, sub="", colour=None):
        """Every page opens with a title and one line. A large figure stands beside them
        only where it says something: a step's number, a tick at the end, a warning when
        something stopped. Four rows either way, so every page's body starts on one row."""
        indent = 0
        if glyph is not None:
            for i, line in enumerate(big(glyph)):
                self.put(self.y + i, self.x, line, colour or self.accent)
            indent = len(big(glyph)[0]) + 3
        self.put(self.y, self.x + indent, title, PAPER, bold=True)
        for i, line in enumerate(wrap(sub, self.w - indent)[:2]):
            self.put(self.y + 1 + i, self.x + indent, line, LICHEN)
        self.y += 4
        return self

    def text(self, text, colour=PAPER, bold=False, indent=0):
        for line in wrap(text, self.w - indent):
            self.put(self.y, self.x + indent, line, colour, bold=bold)
            self.y += 1
        return self

    def hint(self, parts):
        """What the next key does: keys as keycaps inside a dim sentence, wrapped to the pane."""
        col = self.x
        for part in parts:
            if isinstance(part, tuple):
                cap = f" {part[0]} "
                if col + len(cap) > self.x + self.w:
                    self.y, col = self.y + 1, self.x
                col = self.put(self.y, col, cap, PAPER, RULE, bold=True)
                continue
            for i, word in enumerate(re.split(r"(?<= )", part)):
                if col + len(word.rstrip()) > self.x + self.w:
                    self.y, col = self.y + 1, self.x
                    word = word.lstrip()
                col = self.put(self.y, col, word, LICHEN)
        self.y += 1
        return self

    def section(self, title, trailing=""):
        col = self.put(self.y, self.x, title, self.accent, bold=True)
        end = self.x + self.w - (len(trailing) + 1 if trailing else 0)
        self.put(self.y, col + 1, "─" * max(0, end - col - 1), HAIR)
        if trailing:
            self.put(self.y, self.x + self.w - len(trailing), trailing, LICHEN)
        self.y += 1
        return self

    def facts(self, rows, x=None, bg=None, value_colour=PAPER):
        x = self.x if x is None else x
        width = max(len(label) for label, _ in rows)
        for label, value in rows:
            self.put(self.y, x, label.ljust(width), LICHEN, bg)
            self.runs(self.y, x + width + 2, value if isinstance(value, list) else [(value, value_colour)], bg)
            self.y += 1
        return self

    def panel(self, tone, title, lines=(), facts=None, x=None, w=None, keys=None):
        """A status that must not be missed: a raised block with a solid edge in the
        tone's colour. `keys` adds keycaps on its last line (a yes-or-no)."""
        x, w = self.x if x is None else x, self.w if w is None else w
        colour = TONES.get(tone, tone)
        body = [line for text in lines for line in wrap(text, w - 4)]
        height = 1 + len(body) + (len(facts) if facts else 0) + (1 if keys else 0)
        top = self.y
        for r in range(top, top + height):
            self.fill(r, x, w, RAISED)
            self.put(r, x, "█", colour)
        self.put(top, x + 2, f"{TONE_MARK.get(tone, '●')} {title}", colour, RAISED, bold=True)
        row = top + 1
        for line in body:
            self.put(row, x + 2, line, PAPER, RAISED)
            row += 1
        if facts:
            width = max(len(label) for label, _ in facts)
            for label, value in facts:
                self.put(row, x + 2, label.ljust(width), LICHEN, RAISED)
                self.put(row, x + 4 + width, value, PAPER, RAISED)
                row += 1
        if keys:
            col = x + 2
            for key, label in keys:
                col = self.put(row, col, f" {key} ", NIGHT, colour, bold=True)
                col = self.put(row, col + 1, label, PAPER, RAISED) + 3
            row += 1
        self.y = row + 1
        return self

    def cards(self, cards, height=None):
        """Choices side by side: (title, tag, lines, chosen). The chosen one has the
        accent edge and a filled mark."""
        gap = 2
        w = (self.w - gap * (len(cards) - 1)) // len(cards)
        bodies = [[line for text in lines for line in wrap(text, w - 4)] for _, _, lines, _ in cards]
        height = height or 1 + max(len(body) for body in bodies)
        for i, (title, tag, _, chosen) in enumerate(cards):
            x = self.x + i * (w + gap)
            for r in range(self.y, self.y + height):
                self.fill(r, x, w, RAISED)
                self.put(r, x, "█", self.accent if chosen else RULE)
            col = self.put(self.y, x + 2, ("● " if chosen else "○ ") + title, self.accent if chosen else PAPER, RAISED, bold=True)
            if tag:
                self.put(self.y, x + w - 2 - len(tag) - 2, f" {tag} ", NIGHT if chosen else LICHEN, self.accent if chosen else RULE, bold=chosen)
            for j, line in enumerate(bodies[i]):
                self.put(self.y + 1 + j, x + 2, line, MOSS if chosen else LICHEN, RAISED)
        self.y += height + 1
        return self

    def table(self, headers, rows, aligns=None, colours=None, selected=None, x=None):
        """Data in columns: a dim header, rows in the text colour; `colours` tints a row,
        `selected` draws one as the selection bar."""
        x = self.x if x is None else x
        aligns = aligns or ["l"] * len(headers)
        widths = [max(len(str(headers[c])), *(len(str(row[c])) for row in rows)) for c in range(len(headers))]
        def line(cells):
            return "  ".join(str(cell).rjust(widths[c]) if aligns[c] == "r" else str(cell).ljust(widths[c]) for c, cell in enumerate(cells))
        self.put(self.y, x, line(headers), GUTTER, bold=True)
        self.y += 1
        for i, row in enumerate(rows):
            if i == selected:
                self.fill(self.y, x - 1, self.w + 1 - (x - self.x), self.accent)
                self.put(self.y, x, line(row), NIGHT, self.accent, bold=True)
            else:
                self.put(self.y, x, line(row), (colours[i] if colours and colours[i] else PAPER))
            self.y += 1
        return self

    def commands(self, lines, label=""):
        """Commands as a block of the darker surface, each behind a prompt."""
        body = [(i == 0, part) for text in lines for i, part in enumerate(wrap(text, self.w - 6))]
        for r in range(self.y, self.y + len(body)):
            self.fill(r, self.x, self.w, INK)
        if label:
            self.put(self.y, self.x + self.w - len(label) - 2, label, GUTTER, INK)
        for first, part in body:
            if first:
                self.put(self.y, self.x + 2, "$", self.accent, INK, bold=True)
            self.put(self.y, self.x + 4, part, SAND if self.accent != SAND else SAGE, INK)
            self.y += 1
        return self

    def steplist(self, groups, x=None):
        """Commands or checks with how each went: (phase, [(state, text)])."""
        x = self.x if x is None else x
        for phase, items in groups:
            if phase:
                self.put(self.y, x, phase, LICHEN, bold=True)
                self.y += 1
            for state, text in items:
                mark, colour = MARKS[state]
                self.put(self.y, x + 1, mark, colour or self.accent)
                self.put(self.y, x + 4, text[: self.w - 5], PAPER if state != "todo" else MOSS)
                self.y += 1
        return self

    def bar(self, fraction, label=""):
        """How far a long job has got: a thick bar, then the figure."""
        figure = f" {int(fraction * 100):3d}%"
        cells = self.w - len(figure)
        filled = round(cells * fraction)
        self.put(self.y, self.x, "█" * filled, self.accent)
        self.put(self.y, self.x + filled, "█" * (cells - filled), RULE)
        self.put(self.y, self.x + cells, figure, PAPER, bold=True)
        self.y += 1
        return self

    def field(self, label, text, trailing="", width=None, secret=False, x=None, cursor=True):
        """Something being typed: a raised row with a cursor."""
        x, width = self.x if x is None else x, width or self.w
        self.fill(self.y, x, width, RAISED)
        col = self.put(self.y, x + 1, label, GUTTER, RAISED)
        col = self.put(self.y, col + 2, "•" * len(text) if secret else text, PAPER, RAISED, bold=True)
        if cursor:
            self.grid[self.y][col]["bg"] = self.accent
        if trailing:
            self.put(self.y, x + width - 1 - len(trailing), trailing, GUTTER, RAISED)
        self.y += 1
        return self

    def rows_list(self, rows, selected, width, match="", x=None):
        """A list to choose from: (dim prefix, name, detail, quiet detail) rows; the
        letters a search matched are picked out; one row is the selection bar."""
        x = self.x if x is None else x
        for i, (prefix, name, detail, quiet) in enumerate(rows):
            label = prefix + name
            if i == selected:
                self.fill(self.y, x, width, self.accent)
                self.put(self.y, x + 1, prefix, NIGHT, self.accent)
                self.put(self.y, x + 1 + len(prefix), name, NIGHT, self.accent, bold=True)
                if detail:
                    self.put(self.y, x + width - 2 - len(detail) - len(quiet) - 2, detail, NIGHT, self.accent, bold=True)
                    self.put(self.y, x + width - 1 - len(quiet), quiet, NIGHT, self.accent)
            else:
                self.put(self.y, x + 1, prefix, LICHEN)
                self.put(self.y, x + 1 + len(prefix), name, PAPER)
                m = re.search(re.escape(match), label, re.I) if match else None
                if m:
                    self.put(self.y, x + 1 + m.start(), label[m.start():m.end()], self.highlight, bold=True)
                if detail:
                    self.put(self.y, x + width - 2 - len(detail) - len(quiet) - 2, detail, MOSS)
                    self.put(self.y, x + width - 1 - len(quiet), quiet, GUTTER)
            self.y += 1
        return self

    def dim(self, col_from, col_to, amount=0.55):
        """A pane that does not have the keyboard: its colours fall back toward the surface."""
        for row in self.grid:
            for cell in row[col_from:col_to]:
                cell["fg"] = mix(cell["fg"], cell["bg"], amount)
                if cell["bg"] not in (NIGHT, INK, RAISED):
                    cell["bg"] = mix(cell["bg"], INK, amount)
        return self


# ====================================================================== second pass
# Parts made for one kind of page: each page gets the picture of what it is doing.

PIXELS["%"] = ["#...#", "...#.", "..#..", "..#..", ".#...", "#...#"]
TINTS = [SAGE, LEAF, MOSS, SAND, GUTTER, SALMON]


class Deliberate(Page):
    """A page with the parts of the second pass."""

    def card(self, x, y, w, h, bg=INK):
        for r in range(y, y + h):
            self.fill(r, x, w, bg)
        return self

    def button(self, label, key="Enter", colour=None):
        """The page's one action: a bar in the accent with its key at the end, always on
        the same row near the foot of the page."""
        colour = colour or self.accent
        row = self.rows - 5
        self.fill(row, self.x, self.w, colour)
        self.put(row, self.x + 2, label, NIGHT, colour, bold=True)
        self.put(row, self.x + self.w - len(key) - 4, f" {key} ", colour, NIGHT, bold=True)
        return self

    def note(self, text, colour=LICHEN):
        """One quiet line on the action's row, on a page that has no bar to press."""
        self.put(self.rows - 5, self.x, text[: self.w], colour)
        return self

    def diskbar(self, parts, total, x=None, width=None, label="", selected=None, row=None):
        """A disk as one bar. parts: (name, size, colour); names sit in their stretch
        when it is wide enough. `selected` puts a caret under one."""
        x, width = self.x if x is None else x, width or self.w
        row = self.y if row is None else row
        if label:
            self.put(row, x, label.ljust(7), LICHEN)
            x, width = x + 7, width - 7
        col = x
        spans = []
        for i, (name, size, colour) in enumerate(parts):
            w = max(2, round(width * size / total))
            w = min(w, x + width - col - 2 * (len(parts) - 1 - i))
            self.fill(row, col, w, colour)
            if len(name) + 2 <= w:
                self.put(row, col + 1, name, NIGHT, colour, bold=True)
            spans.append((col, w))
            col += w
        if col < x + width:
            self.fill(row, col, x + width - col, RULE)
        if selected is not None:
            start, w = spans[selected]
            self.put(row + 1, start, "▀" * w, PAPER)
        if row == self.y:
            self.y += 2 if selected is not None else 1
        return spans

    def gauge(self, used, total, text, x=None, width=None):
        """How much of something a thing takes: the taken part in the accent."""
        x, width = self.x if x is None else x, width or self.w
        taken = max(1, round(width * used / total))
        self.fill(self.y, x, taken, self.accent)
        self.fill(self.y, x + taken, width - taken, RULE)
        self.put(self.y + 1, x, text, LICHEN)
        self.y += 3
        return self

    def figure(self, text, colour=None, x=None, row=None):
        """A number three rows tall."""
        x, row = self.x if x is None else x, self.y if row is None else row
        glyphs = [PIXELS[ch] for ch in text]
        for r in range(3):
            line = " ".join("".join("█" if px[r * 2][c] == "#" and px[r * 2 + 1][c] == "#" else "▀" if px[r * 2][c] == "#" else "▄" if px[r * 2 + 1][c] == "#" else " " for c in range(5)) for px in glyphs)
            self.put(row + r, x, line, colour or self.accent)
        return len(glyphs) * 6 - 1

    def checks(self, columns, x=None, width=None):
        """Checks in columns under a small label each: (label, [(state, text)])."""
        x, width = self.x if x is None else x, width or self.w
        w = width // len(columns)
        top = self.y
        for i, (label, items) in enumerate(columns):
            self.put(top, x + i * w, label.upper(), GUTTER, bold=True)
            for j, (state, text) in enumerate(items):
                mark, colour = MARKS[state]
                self.put(top + 1 + j, x + i * w, mark, colour or self.accent, bold=state == "failed")
                self.put(top + 1 + j, x + i * w + 2, text, SALMON if state == "failed" else PAPER)
        self.y = top + 1 + max(len(items) for _, items in columns) + 1
        return self

    def filediff(self, title, lines, x=None, width=None):
        """A file as it will be written: line numbers, lines going out and coming in."""
        x, width = self.x if x is None else x, width or self.w
        self.fill(self.y, x, width, RAISED)
        self.put(self.y, x + 2, title, MOSS, RAISED, bold=True)
        self.y += 1
        for kind, number, text in lines:
            colour = {"+": LEAF, "-": SALMON, " ": LICHEN}[kind]
            self.fill(self.y, x, width, INK)
            self.put(self.y, x + 1, str(number).rjust(2), GUTTER, INK)
            self.put(self.y, x + 5, kind, colour, INK, bold=True)
            self.put(self.y, x + 7, text[: width - 9], colour if kind != " " else MOSS, INK)
            self.y += 1
        return self

    def wordslots(self, word, typed, x=None):
        """A word to type, a cell a letter: the destructive steps' last gate."""
        x = self.x if x is None else x
        done = typed == word
        for i, ch in enumerate(word):
            filled = i < len(typed)
            bg = (LEAF if done else SALMON) if filled else RAISED
            self.fill(self.y, x + i * 4, 3, bg)
            self.put(self.y, x + i * 4 + 1, typed[i] if filled else "·", NIGHT if filled else GUTTER, bg, bold=True)
        end = x + len(word) * 4 + 2
        self.put(self.y, end, "unlocked" if done else "locked", LEAF if done else SALMON, bold=True)
        self.y += 2
        return self

    def keyboard(self, rows, x, y):
        """The letter rows of a keyboard, a cell a key."""
        for r, keys in enumerate(rows):
            col = x + r
            for key in keys.split():
                self.put(y + r, col, f" {key} ", PAPER, RAISED, bold=True)
                col += 4
        return self

    def meter(self, fraction, text, x=None, width=20):
        x = self.x if x is None else x
        filled = round(width * fraction)
        colour = SALMON if fraction < 0.4 else SAND if fraction < 0.7 else LEAF
        self.put(self.y, x, "━" * filled, colour, bold=True)
        self.put(self.y, x + filled, "━" * (width - filled), RULE)
        self.put(self.y, x + width + 2, text, LICHEN)
        self.y += 1
        return self

    def timeline(self, items, x=None):
        """What happened, newest first, down a line: (time, state, title, detail)."""
        x = self.x if x is None else x
        for i, (when, state, title, detail) in enumerate(items):
            mark, colour = MARKS[state]
            self.put(self.y, x, when, GUTTER)
            self.put(self.y, x + 10, mark, colour or self.accent, bold=True)
            self.put(self.y, x + 13, title, PAPER, bold=True)
            self.put(self.y + 1, x + 10, "│" if i < len(items) - 1 else " ", HAIR)
            self.put(self.y + 1, x + 13, detail, LICHEN)
            self.put(self.y + 2, x + 10, "│" if i < len(items) - 1 else " ", HAIR)
            self.y += 3
        return self

    def slotbox(self, x, y, w, h, name, lines, tag="", state="idle"):
        """A system slot as a block: its name, what it holds, and its part in the next boot."""
        colour = {"running": self.accent, "next": self.highlight, "idle": RULE}[state]
        self.card(x, y, w, h, RAISED)
        self.fill(y, x, w, colour)
        self.put(y, x + 2, name, NIGHT if state != "idle" else PAPER, colour, bold=True)
        if tag:
            self.put(y, x + w - len(tag) - 2, tag, NIGHT if state != "idle" else LICHEN, colour, bold=state != "idle")
        for i, (text, tone) in enumerate(lines):
            self.put(y + 2 + i, x + 2, text, tone, RAISED)
        return self

    def arrow(self, row, col, text="", length=4, colour=None):
        colour = colour or GUTTER
        col = self.put(row, col, "─" * length + "▶", colour)
        return self.put(row, col + 1, text, PAPER) if text else col

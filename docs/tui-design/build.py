"""Writes the design board: the rules, then every page of every app as a mock-up."""
import html, sys
sys.path.insert(0, ".")
from pages_system import PAGES

APPS = [
    ("install screen", "The install screen", "What the live ISO opens. Today it is two boxed panes, each app with its own header and footer, and a third footer for the screen."),
    ("install-hub", "Install steps", "The left pane of the install screen: the six steps in order."),
    ("disk-plan", "Disk", "Choosing, reviewing and writing the disk layout, on top of recpart."),
    ("base-system", "Base system", "Copying the system and writing its fstab, on top of recstrap and recfstab."),
    ("system-setup", "Configuration", "Keyboard, time zone, language, hostname, passwords and a user, on top of recsetup."),
    ("recab", "Updates", "The installed system's A/B slots, on top of recab."),
    ("guide", "Guide", "The docs, beside the install steps or on their own."),
]
RULES = [
    ("Every page has its own picture", "Inside the shared frame, each page is laid out for the one thing it does: a disk is a bar, a choice between two futures is two bars, a copy is a figure three rows tall, a file about to be written is that file, a keyboard is its keys, a time zone is a place on a map, a trial boot is a fork you can follow."),
    ("One frame", "Every screen is the same five parts in the same places: the band (whose system, which app, where you are), the step line, the rail, the content, the keys. Nothing is boxed; parts are told apart by their surface."),
    ("One opening", "Every page opens with a large glyph, a title and one line. The glyph is the step's number on a step, a diamond on an overview, a tick when the app is done, a warning when something stopped."),
    ("One rail", "The rail is always the same rows: a mark, a name, and what was answered. Steps in order are joined by a line; things to choose from are grouped under a small label. The row you are on is a raised bar with the accent at its edge."),
    ("One set of marks", "✓ done, ◉ you are here, ● ready, ○ not yet, ◐ running, ✗ failed or refused. They mean the same in every app."),
    ("One way to say how it stands", "A verdict is a raised panel with a solid edge in its tone: green for done, sand for a question or a warning, salmon for a refusal or a failure, grey-green for a note. It sits directly under the opening."),
    ("One way to ask", "A yes-or-no is the warning panel with its two keys inside it. Something destructive also asks for a typed word, in a field. A choice between two is two cards; a choice from many is a search field and a list."),
    ("One way to show data", "Facts are label and value. Tables have a quiet header. Commands sit on the darker surface behind a prompt. A long job is a thick bar and its figures."),
    ("One ending", "Every app ends on the same page: the large tick, what was done, and the next step."),
    ("One line of keys", "The keys are keycaps at the bottom left, in the order you would press them; what the screen is working on is at the bottom right."),
    ("Two brands", "LevitateOS is pine and sage, AcornOS acorn brown and sand. Only the band, the accent and the second accent change."),
]


def screen(p):
    rows = []
    for line in p.grid:
        runs, last = [], None
        for cell in line:
            style = (cell["fg"], cell["bg"], cell["bold"])
            if style == last:
                runs[-1][1].append(cell["ch"])
            else:
                runs.append((style, [cell["ch"]]))
                last = style
        rows.append("".join(
            f'<span style="color:{fg};background:{bg}{";font-weight:700" if bold else ""}">{html.escape("".join(chars))}</span>'
            for (fg, bg, bold), chars in runs))
    return '<pre class="term">' + "\n".join(rows) + "</pre>"


body = []
for key, title, about in APPS:
    pages = [entry for entry in PAGES if entry["app"] == key]
    body.append(f'<section id="{key.replace(" ", "-")}"><h2>{html.escape(title)}</h2><p class="about">{html.escape(about)} {len(pages)} pages.</p>')
    for entry in pages:
        body.append(f'<figure><figcaption><b>{html.escape(entry["name"])}</b> {html.escape(entry["note"])}</figcaption><div class="frame">{screen(entry["page"])}</div></figure>')
    body.append("</section>")

open(sys.argv[1], "w").write(f"""<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>TUI design board</title>
<style>
  :root {{ --paper: #F2EEE4; --ink: #14221A; --pine: #214438; --line: #BFC6B8; --secondary: #59645B; }}
  body {{ margin: 0; background: var(--paper); color: var(--ink); font: 16px/1.5 "Instrument Sans", system-ui, sans-serif; }}
  main {{ max-width: 1360px; margin: 0 auto; padding: 32px 16px 96px; }}
  h1 {{ font: 500 40px/1.05 Newsreader, Georgia, serif; letter-spacing: -0.02em; margin: 0 0 12px; }}
  h2 {{ font: 500 28px/1.1 Newsreader, Georgia, serif; margin: 72px 0 4px; border-top: 1px solid var(--line); padding-top: 24px; }}
  p {{ margin: 0 0 12px; max-width: 72ch; color: var(--secondary); }}
  nav {{ display: flex; flex-wrap: wrap; gap: 8px 20px; margin: 20px 0 8px; font-size: 14px; }}
  nav a {{ color: var(--pine); }}
  ol.rules {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 16px 32px; padding: 0; margin: 24px 0 0; list-style: none; counter-reset: rule; }}
  ol.rules li {{ counter-increment: rule; font-size: 14px; color: var(--secondary); }}
  ol.rules b {{ display: block; color: var(--ink); font-size: 15px; }}
  ol.rules b::before {{ content: counter(rule) "  "; color: var(--pine); }}
  figure {{ margin: 32px 0 0; }}
  figcaption {{ font-size: 14px; color: var(--secondary); margin-bottom: 8px; max-width: 80ch; }}
  figcaption b {{ color: var(--ink); margin-right: 8px; }}
  .frame {{ overflow-x: auto; }}
  .term {{ margin: 0; display: inline-block; font: 13px/1.25 "JetBrains Mono", "Cascadia Mono", "SF Mono", Menlo, Consolas, "DejaVu Sans Mono", monospace; font-variant-ligatures: none; }}
  .term span {{ display: inline-block; white-space: pre; }}
</style></head><body><main>
<h1>Every screen, one design</h1>
<p>{len(PAGES)} pages across the install screen and six apps, drawn as mock-ups from one set of parts, so that no page can drift from another. This is the second pass: the frame and the rules are the first pass's; inside them every page now has a layout of its own. They are designs, not captures: every value in them is sample data, and nothing here is built beyond what system-setup already has.</p>
<nav>{''.join(f'<a href="#{key.replace(" ", "-")}">{html.escape(title)}</a>' for key, title, _ in APPS)}</nav>
<ol class="rules">{''.join(f'<li><b>{html.escape(name)}</b>{html.escape(text)}</li>' for name, text in RULES)}</ol>
{''.join(body)}
</main></body></html>""")
print(len(PAGES), "pages")

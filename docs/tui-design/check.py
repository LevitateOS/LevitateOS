"""Prints the mock-up pages as text and flags content that runs past its pane."""
import sys
sys.path.insert(0, ".")
import importlib
for module in sys.argv[2:]:
    pages = importlib.import_module(module).PAGES
    for entry in pages:
        p = entry["page"]
        over = p.y - 1 > p.rows - 4
        flag = "  OVERFLOW by %d" % (p.y - 1 - (p.rows - 4)) if over else ""
        print(f"## {entry['app']} / {entry['name']}{flag}")
        if sys.argv[1] == "all" or (sys.argv[1] == "over" and over) or sys.argv[1] in entry["name"]:
            print("\n".join("   " + "".join(c["ch"] for c in row).rstrip() for row in p.grid))

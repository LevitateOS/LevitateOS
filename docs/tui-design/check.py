"""Prints the mock-up pages as text and flags content that runs past its pane.

  python3 -B -I check.py all pages_system            every page
  python3 -B -I check.py app:disk-plan pages_system  one app's pages (install screen, install-hub,
                                                     disk-plan, base-system, system-setup, recab, guide)
  python3 -B -I check.py "2 Time zone" pages_system  pages whose name contains the text
  python3 -B -I check.py over pages_system           only pages whose content overflows"""
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
        wanted = sys.argv[1]
        if wanted == "all" or (wanted == "over" and over) or wanted == "app:" + entry["app"] or wanted in entry["name"]:
            print("\n".join("   " + "".join(c["ch"] for c in row).rstrip() for row in p.grid))

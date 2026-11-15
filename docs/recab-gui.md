# recab GUI

`tools/gui/recab` builds `recab-gui`, the desktop window for recab. It follows `docs/product-handoff.md` (sections 2.2 and 2.8) and [recab's contract](../tools/recab/REQUIREMENTS.md): the booted root is locked, the other root is unlocked and is where changes are made, and there is no agent inside the window, only a "Copy for agent" button on failure and output views.

It is one window with one page, "The two roots":

- Each root has a card: its letter, and "Booted — locked" or "Other — unlocked". The other root says where it is mounted (or that it is not mounted, or that it cannot be started because a copy into it did not finish). The root that starts at the next boot (the one-time pick, else the boot default) says so.
- Three buttons. **Make the other root a copy of the booted root** asks first (it overwrites the other root), then shows `recab copy` running and its result. **Start the other root at the next boot** runs `recab boot-other`, and becomes **Do not start the other root at the next boot** (`--cancel`) while a one-time pick is set; it is disabled, with a one-line reason, while the other root cannot be started. **Restart now** asks first, then restarts.
- The roots are read in the background (`recab status --json`, schema 11) with a loading state; F5, Ctrl+R and returning to the window read them again. Copying, picking and restarting go through pkexec (`data/org.levitateos.recab.policy`, one action for `/usr/bin/recab`).

Nothing else is in it: no history, no updates, no software installation (that is Recipe's), no pages about a failed boot. The Disk app (`tools/gui/disk`) shows a "The two roots" comparison page only on a computer that has two roots (recab can be run and the kernel command line has `recab.immutable`).

Details, the stand-ins and the story that checks it: `tools/gui/recab/README.md`. Checked offscreen against stand-ins only; not run on a real A/B computer.

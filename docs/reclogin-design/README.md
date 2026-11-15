# reclogin: the screens

The twelve screens the owner accepted on 2026-11-15, as the source files of the
design canvas. They are the reference for what the greeter draws. Each
`*.dc.html` file is one screen at 1920×1080; `canvas.json` lists them with
their titles.

Read them as markup: every position, size, colour and text is in the inline
styles. The login content sits in a box 1420 wide and 800 high, at 250 from the
left and 140 from the top, inside the frame art.

The two images the files name are the website's art, not stored here:

| In the files | The file |
|---|---|
| `/_blob/a8001a0ed75d91b2bed91dbe057d8fae` | `docs/website/public/art/levitate-frame.webp` |
| `/_blob/03951351796b3bb0e9064d60b1a8b86b` | `docs/website/public/art/acorn/frame-hero.webp` |

Colours are each OS's `[look.dark]` values in `distro/variants/<id>/distro.toml`;
the red of the main button is the website's action colour. The typefaces are
Newsreader (name, clock, greeting, user names) and Instrument Sans (everything
else). Names in the screens are invented.

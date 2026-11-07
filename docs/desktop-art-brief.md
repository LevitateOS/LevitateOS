# Brief for an image-generation agent: desktop wallpapers

Hand this whole file to the agent as its instructions. It is self-contained.

---

You are making the default desktop wallpapers for two Linux distributions that
share one visual identity, "Field Notes": **LevitateOS** (KDE Plasma desktop)
and its sibling **AcornOS** (LXQt desktop). You produce artwork only. Logos,
text and interface are added by the system, never painted into the image.

## Read first

- `docs/website/design/field-notes-v3/levitate-frame-original.png`: the
  existing LevitateOS artwork, a frame of dark pine-green stone and blackened
  green metal with leaf-vein relief, warm amber edge light and one small
  lacquer-red inlay. Your work must look like it belongs to the same family:
  same materials, light and restraint. Do not copy its layout. It is a frame
  with an empty centre; you are making full-bleed wallpapers.
- `docs/website/design/field-notes-v3/GENERATION.md`: the brief that produced
  it.
- `docs/website/public/brand/levitate-ink.svg` and `acorn-ink.svg`: the two
  marks, a hovering leaf above a curved ground line and a tilted, sculpted
  acorn. They are for recognising the motifs only. **Do not draw either mark,
  or any logo, in the artwork.**

## Palette

| Role | Hex |
|---|---|
| Ink (deepest dark) | `#14221A` |
| Pine, LevitateOS | `#214438` |
| Pale pine, LevitateOS on dark | `#C7D9BA` |
| Acorn brown | `#644532` |
| Pale acorn, AcornOS on dark | `#E4CBA6` |
| Paper | `#F2EEE4` |
| Red accent (at most one small touch per image) | `#B22F27` |
| Amber light | a warm material light grazing edges, never a flat UI colour |

## The four images

Two per distribution: a dark one (the default) and a light one (Plasma swaps
to it when the user picks a light colour scheme).

1. **LevitateOS, dark.** Substantial planes of dark pine-green stone and
   blackened green metal, meeting at precise asymmetric bevels and deep shadow
   cuts. Large, restrained leaf-vein relief carved into one or two of the
   planes. Warm amber light grazes a few edges. A sense of something resting
   just above the ground. The leaf "levitates", but show this through light and
   shadow (a thin band of light under a plane), not a floating object. Mood:
   calm, intimidating competence, warmth, weight, exactness.
2. **LevitateOS, light.** The same scene and materials translated to warm paper
   and pale limestone (`#F2EEE4` and slightly darker), with the relief and
   bevels in soft pine shadow. Low contrast overall: dark text and icons must
   stay readable anywhere on it.
3. **AcornOS, dark.** The same architectural language in a different
   material: dark oiled oak and walnut planes, carved bark grain, and as relief
   the overlapping scale pattern of an acorn cup (cupule). Browns from ink
   through `#644532`, amber light on the edges. **No leaves and no leaf veins**.
   Those belong to LevitateOS. Slightly smaller and simpler in feel than
   LevitateOS ("a smaller, simpler base"), with fewer planes and more quiet
   surface.
4. **AcornOS, light.** Image 3 translated to pale wood and paper (`#F2EEE4`,
   `#E4CBA6`), relief in soft brown shadow, low contrast.

## Composition rules

Windows and a taskbar sit on top of a wallpaper, and screens of many shapes crop
it. So:

- **Format:** 16:9 landscape. Target 3840×2160. If your tool cannot produce
  at least 2560×1440, say so and deliver the largest it can. Do not upscale
  it yourself.
- **Crop-safe:** the image must still look intentional when cropped to the
  centre 4:3 area and to the centre 21:9 band. Put no essential detail in the
  outer 12% at left and right.
- **Quiet zones:** the bottom 6% is covered by the panel. Keep it plain and
  even. The upper-left quarter may hold desktop icons with white labels on the
  dark images and dark labels on the light ones; keep it low in detail.
- **Where the detail goes:** concentrate the richest material and relief in one
  area off centre (for example lower right into centre), and let the rest fall
  away into large calm planes. The middle of the screen is where windows open,
  so avoid strong focal points or high-contrast texture there.
- **Restraint:** no busy pattern, no repeating tiles, no noise or grain strong
  enough to show at 100% zoom. Detail should read at full screen and still
  look calm at a thumbnail.
- **Never:** text, letters, numbers, logos or marks, user-interface elements,
  people, animals, signatures, watermarks, borders or frames around the image,
  lens flare, bokeh, neon, gradients that band.

## Deliverables

Write everything into `.artifacts/desktop-art/` in the LevitateOS checkout (it
is git-ignored; the user picks the final images from there):

- For each of the four images, **three candidates**:
  `levitate-dark-1.png` … `levitate-dark-3.png`, `levitate-light-1.png` …,
  `acorn-dark-1.png` …, `acorn-light-1.png` ….
  PNG, sRGB, metadata stripped, no other processing (no crop, retouching or
  colour grading after generation).
- `GENERATION.md` in the same folder, in the same shape as
  `docs/website/design/field-notes-v3/GENERATION.md`: the tool and model used,
  the exact prompt for each image, the size each file actually came out at, and
  anything you could not meet from this brief.

Do not edit anything else in the repository.

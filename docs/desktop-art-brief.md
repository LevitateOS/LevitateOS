# Brief for an image-generation agent: desktop wallpapers (round 2)

Hand this whole file to the agent as its instructions. It is self-contained.

---

You are making the default desktop wallpapers for two Linux distributions:
**LevitateOS** (KDE Plasma desktop) and its sibling **AcornOS** (LXQt
desktop). You produce artwork only. Logos, text and interface are added by the
system, never painted into the image.

## What went wrong in round 1

Round 1 (`.artifacts/desktop-art/*.png`) asked for minimal, dark, architectural
stone-and-wood studies. The user rejected all twelve as ugly and unsettling. Do
not reuse that direction: no brooding empty planes, no cold stone, no uncanny
or oppressive mood, no minimalism. These are **scenes**: rich, detailed,
warm, places you would want to be. Peaceful and inviting, never eerie.

## Three styles

Every scene is made in all three styles. Keep the scene and composition the
same across the three, so they read as one picture painted three ways.

1. **Realistic.** A photograph: natural light, real materials, believable depth
   of field. Not a 3D render, not over-processed HDR, not oversaturated.
2. **Western professionally drawn.** Hand-drawn and painted illustration in
   the Western tradition: a skilled storybook, editorial or animation
   background painter (gouache, watercolour or painterly digital with visible,
   confident drawing). Not anime or manga, not cel-shaded, not clip art, not
   vector-flat.
3. **Dutch abstract.** In the spirit of De Stijl (Mondrian, van Doesburg): the
   scene reduced to horizontal and vertical black lines and flat rectangular
   colour planes. Mondrian's own tree studies, which move step by step from a
   painted tree to pure grid, show how a landscape becomes this language. The
   scene must still be recognisable in its structure (the treehouse, the row of
   trees, the horizon; the table, the trunks). Do not copy any existing
   painting. You may use the brand colours below in place of pure primaries.

## LevitateOS: the treehouse (6 images)

A **treehouse** in a large, healthy tree, with **no people**. Behind it lies a
**farm**: fields, perhaps a barn or fences, nothing industrial. Far in the
background a **row of trees** runs along the horizon. **The sky is visible**
and takes a real share of the picture.

In the **foreground**, a single **leaf hovers just above the ground**. It is
about to land and doesn't: it fails, or refuses, to touch down. Show a clear
small gap between leaf and ground, with its shadow on the grass below it. This
is the heart of the LevitateOS identity (the logo is a leaf above a curved
ground line). It should be noticeable but not huge, off centre, and quietly
impossible rather than magical: no glow, no sparkles.

Two versions of each style:

- **Light: daytime, nice weather.** Clear or lightly clouded sky, soft sun,
  green summer fields.
- **Dark: night.** The same scene lit by **soft artificial light**: warm
  lamplight in the treehouse windows, perhaps a lantern or a string of small
  lights along the treehouse. Stars or a deep blue sky. Calm and cosy, still
  clearly readable, not pitch black.

Colour lean: greens toward pine `#214438` and pale pine `#C7D9BA`; warm
lamplight toward amber.

## AcornOS: the picnic table (3 images)

Deep inside a **forest in deep autumn**: rich reds, burnt orange, ochre and
brown leaves, on the trees and thick on the ground. A wooden **picnic table**
stands in a clearing, **not centred**.

A **family of squirrels is rushing** past or around the table. Show them
with **motion blur**, as if the photograph used a longer exposure (slow
shutter): the table, trunks and leaves are sharp, and the squirrels are
streaks of movement that are still clearly squirrels. **No acorns are visible
anywhere**, yet the viewer should sense the squirrels are after acorns that are
close by: their direction, urgency and where they're heading (one diving into
the leaf litter, one darting toward the base of a large oak) tell the story.

In the drawn style, show the motion with the painter's means (drybrush
streaks, repeated outlines, speed in the brushwork). In the Dutch abstract
style, show it with a staccato run of small coloured rectangles stepping
along the grid lines (the device of Mondrian's late New York work), heading
toward one spot.

Only one version per style. LXQt does not switch wallpaper with a light/dark
scheme, so it needs no separate night image.

Colour lean: browns toward acorn brown `#644532` and pale acorn `#E4CBA6`,
with autumn reds and orange.

## Brand colours (for the abstract style, and as a lean elsewhere)

| Role | Hex |
|---|---|
| Ink (deepest dark, the De Stijl black lines) | `#14221A` |
| Pine, LevitateOS | `#214438` |
| Pale pine | `#C7D9BA` |
| Acorn brown | `#644532` |
| Pale acorn | `#E4CBA6` |
| Paper (the De Stijl white) | `#F2EEE4` |
| Red | `#B22F27` |

## Composition rules (all images)

A wallpaper sits behind windows and a taskbar, and screens of different shapes
crop it.

- **Format:** 16:9 landscape, at the largest native size your tool produces.
  Report the actual size; do not upscale (that is a separate step).
- **Crop-safe:** the image must still work when cropped to the centre 4:3 area
  and to the centre 21:9 band. Keep the key subjects (treehouse, hovering
  leaf, picnic table, squirrels) out of the outer 12% at left and right and out
  of the top and bottom 8%.
- **Taskbar:** the bottom 6% sits under the panel. Nothing important there; the
  hovering leaf must sit clearly above it.
- **Readability:** desktop icons may sit in the upper-left quarter. Keep that
  area moderate in detail (sky or canopy work well).
- **Never:** text, letters, numbers, logos, user-interface elements, people,
  signatures, watermarks, borders or frames, lens flare. No visible acorns in
  the AcornOS images. No people in any image.

## Deliverables

Write everything into `.artifacts/desktop-art/round-2/` in the LevitateOS
checkout (git-ignored; the user picks the finals from there). Leave round 1
where it is.

Two candidates per image, 18 files in all:

- `levitate-realistic-light-1.png`, `-2.png`; `levitate-realistic-dark-1.png`, `-2.png`
- `levitate-drawn-light-…`, `levitate-drawn-dark-…`
- `levitate-abstract-light-…`, `levitate-abstract-dark-…`
- `acorn-realistic-1.png`, `-2.png`; `acorn-drawn-…`; `acorn-abstract-…`

PNG, sRGB, metadata stripped, no processing after generation (no crop,
retouching or colour grading).

Also write `GENERATION.md` in that folder: the tool and model used, the exact
prompt for each image, the size each file actually came out at, and anything
from this brief you could not meet.

Do not edit anything else in the repository.

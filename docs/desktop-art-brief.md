# Brief for an image-generation agent: desktop wallpapers (round 3)

Hand this whole file to the agent as its instructions. It is self-contained.
The subjects, styles and feel below are what the user asked for; the
composition and details are yours to work out.

---

You are making the default desktop wallpapers for two Linux distributions:
**LevitateOS** (KDE Plasma desktop) and its sibling **AcornOS** (LXQt
desktop). You produce artwork only. Logos, text and interface are added by the
system, never painted into the image.

## What the user rejected

- **Round 1:** minimal, dark, architectural stone-and-wood studies. Rejected as
  ugly and unsettling. These are **scenes**: rich, detailed and warm, places
  you want to be. Nothing eerie, empty or cold.
- **Round 2:** the scenes were right, but the drawn style looked like a
  children's book, the colours were ordinary and the surfaces looked cheap.
  This round should feel **mature, refined and luxurious**: never cute, naive
  or cartoon.

## Colour and finish

- **Colour:** deep, rich and expensive-looking. Shadows that keep their colour,
  warm golds rather than yellows, nothing washed out or candy-bright. Think of a
  fine library or a luxury catalogue rather than a postcard.
- **Light:** directional, with depth and soft falloff. Nights are deep blue,
  not grey or black.
- **Texture:** surfaces with fine, tactile detail (wood grain, bark, fur, leaf
  veins, the canvas in the painted styles) that holds up at full screen size.
- **Mood:** calm, assured and adult.

## The three styles

Each scene is made in all three styles. Keeping the composition similar
across them is nice but not required.

1. **Realistic.** A detailed, high-end landscape photograph: sharp, rich and
   believable, with real light. Not a 3D render, not HDR-looking.
2. **Painted.** A refined oil painting in the tradition of the 19th-century
   Western landscape painters (the Hudson River School, the Barbizon
   painters). Mature and gallery-worthy, not storybook or cartoon.
3. **Abstract.** Dutch abstraction in the spirit of De Stijl (Mondrian, van
   Doesburg): the scene reduced to horizontal and vertical lines and
   rectangles of colour, with a fine painted surface. The scene should still be
   faintly recognisable, as in Mondrian's tree studies. Copy no existing
   painting. Gold leaf for the lights is a nice touch.

## LevitateOS: the treehouse (3 styles × light and dark)

A **treehouse** in a large tree, with **no people**. Make it look well built
and beautiful, not a kids' den. Behind it is a **farm**, and far in the
background a **row of trees** along the horizon. **The sky is visible.**

In the **foreground**, a **leaf hovers just above the ground**: it fails, or
refuses, to touch down. This is the heart of the LevitateOS identity (the logo
is a leaf above a curved ground line). It should be noticeable but quiet,
impossible rather than magical: no glow or sparkles.

- **Light:** a day with nice weather.
- **Dark:** night, lit by soft artificial light (the treehouse windows, for
  example).

Colours lean toward the brand greens: pine `#214438` and pale pine `#C7D9BA`.

## AcornOS: the picnic table (3 styles × light and dark)

A **picnic table** inside a **forest in deep autumn colours**, **not
centred**.

A **family of squirrels is rushing** through the scene, blurred as if the
photograph had a longer exposure (in the painted and abstract styles, find
each style's own way to show that speed). **No acorns are visible**, but you
can tell the squirrels are after acorns, and that the acorns are close.

- **Light:** autumn daytime.
- **Dark:** dusk or evening, with a soft warm light (a lantern on the table,
  for example).

Colours lean toward acorn brown `#644532` and pale acorn `#E4CBA6` among the
autumn reds and oranges.

## Practical limits

- **Format:** 16:9 landscape at the largest native size your tool produces.
  Report the actual size; do not upscale.
- **Cropping:** screens of other shapes crop the edges, so keep the main
  subjects away from the far left and right.
- **Taskbar and icons:** a panel covers a strip along the bottom, and desktop
  icons sit at the top left, so keep anything important out of those two
  areas.
- **Never:** text, logos, interface elements, people, signatures or
  watermarks.

## Deliverables

Write everything into `.artifacts/desktop-art/round-3/` in the LevitateOS
checkout (git-ignored; the user picks the finals there). Leave earlier rounds
where they are.

Two candidates per image, named `<distro>-<style>-<light|dark>-<n>.png` with
style `realistic`, `painted` or `abstract` (for example
`levitate-painted-dark-2.png`, `acorn-realistic-light-1.png`). PNG, sRGB, no
processing after generation.

Write `GENERATION.md` in that folder: the tool and model used, the prompt for
each image, and the size each file came out at.

Edit nothing else in the repository.

# Website image assignments

Fourteen images for the website: six frames and one link preview background
for AcornOS, and the same seven for LevitateOS. The site shows LevitateOS pages
light (paper and pine green) and AcornOS pages dark (deep brown and sand), so
each distro needs its own set. Today both use the one green set in
`docs/website/public/art/`, which is why AcornOS pages still show green frames.

**AcornOS comes first: its seven are the ones that are missing.** LevitateOS
already has six frames that fit its light pages; its prompts are here so you
can regenerate a matching set if you want one, and for its link preview.

Each assignment is **standalone** and comes in two parts, as in
`docs/desktop-art-brief.md`:

- **Positive:** the brief for the model that writes the image prompt. It
  describes only what should be in the image.
- **Negative:** the negative prompt, ready to paste into ComfyUI as it is.

## What the frames are

A frame is not a picture with a hole in it. The browser uses only the outer
ring of the image, about 154 px wide on a 1536 x 1024 image, cuts it into four
corners and four sides, and stretches the sides around a live page panel.
Everything inside that ring is discarded. That is why every brief asks for
detail in the corners, uniform material along the sides and a plain opening.

Before you keep an image, check:

1. It is straight-on: the opening is a true rectangle, centred, with edges
   parallel to the image edges.
2. The material reaches all four image edges, with no margin or backdrop.
3. The band 154 px in from each edge is all frame, on all four sides.
4. Along each side, between the corners, nothing stands out: no leaf, knot,
   joint or spot that would smear when stretched.
5. No text, numbers or logo.
6. AcornOS only: the frame is clearly lighter than a very dark brown page
   (`#1F1711`). Hold it against that colour.

## Where each one goes

| Slot | Where it is shown | LevitateOS motif | AcornOS motif |
|---|---|---|---|
| hero | Homepage hero | Leaf-vein relief in stone | Oak-leaf relief in walnut and bronze |
| fern | Homepage introduction | Pressed fern fronds | Pressed oak leaves and acorn cups |
| circuit | Around the recipe code | Engraved circuit traces | Dovetail joints and brass inlay |
| mineral | Beside the updates text | Banded green stone | End-grain oak rings |
| paper | Docs: Getting Started | Folded green paper | Folded kraft paper |
| atlas | Downloads page | Contour map in metal | Contour map in leather |
| og | Link preview (1200 x 630) | Wide hero frame | Wide hero frame |

## When you have picked

Save each final image under the name its assignment gives, as PNG at the
generated size, in `.artifacts/website-art/final/`. From there the frames are
encoded to WebP (quality 83, metadata removed, no retouching or crop), each
flavour's pages are pointed at its own set, and the link preview images get the
wordmark and the line added on top. The favicon needs no generated image: it
is made from the existing acorn mark.


## AcornOS

### 1. AcornOS, hero frame: oak-leaf relief in walnut and bronze

Used for: The homepage hero, the largest and most seen. It sets the family look. Save as `acorn-frame-hero.png`.

Positive:

```text
Frame artwork for the AcornOS website: oak-leaf relief in walnut and bronze.

Subject: An architectural aperture. Large, restrained oak-leaf vein relief in the upper-left and lower-right corners, as if a single huge oak leaf were pressed into the wood. In one other corner the cross-hatched scale pattern of an acorn cap, cut shallow into bronze. One small lacquer-red inlay interrupts the material in the lower-right corner.

Material: oiled walnut, oak and dark bronze.

Light: Warm amber light grazes the inner left and lower edges of the opening. The outer edge of the surround also catches a thin sand-coloured highlight.

Feel: small and strict: the fewest parts that do the job. Plain, exact joinery rather than ornament; weight and warmth without decoration for its own sake. The page around and inside it is a very dark brown (#1F1711), so the surround must be clearly lighter than that: mid-tone wood and metal with lit edges, never a near-black frame that disappears into the page.

Colour: a narrow range of warm browns from deep umber through acorn brown (#644532) to pale sand (#E4CBA6) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else, and no green.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain very dark warm brown (#1F1711), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
green, pine green, moss, ferns, stone, marble, malachite, blue, purple, grey, near-black frame, black frame, charcoal, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 2. AcornOS, fern frame: pressed oak leaves and acorn cups

Used for: The homepage's "Nothing hidden" introduction. Save as `acorn-frame-fern.png`.

Positive:

```text
Frame artwork for the AcornOS website: pressed oak leaves and acorn cups.

Subject: Herbarium relief: oak leaves and two or three empty acorn cups pressed into the surface in the corners, with the sides carrying only fine, even, parallel wood grain. Leaves and cups are in the corners; the sides carry only the grain.

Material: oiled walnut, oak and dark bronze.

Light: Warm amber light grazes the inner left and lower edges of the opening. The outer edge of the surround also catches a thin sand-coloured highlight.

Feel: small and strict: the fewest parts that do the job. Plain, exact joinery rather than ornament; weight and warmth without decoration for its own sake. The page around and inside it is a very dark brown (#1F1711), so the surround must be clearly lighter than that: mid-tone wood and metal with lit edges, never a near-black frame that disappears into the page.

Colour: a narrow range of warm browns from deep umber through acorn brown (#644532) to pale sand (#E4CBA6) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else, and no green.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain very dark warm brown (#1F1711), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
green, pine green, moss, ferns, stone, marble, malachite, blue, purple, grey, near-black frame, black frame, charcoal, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 3. AcornOS, circuit frame: dovetail joints and brass inlay

Used for: Around the recipe code sample on the homepage. Save as `acorn-frame-circuit.png`.

Positive:

```text
Frame artwork for the AcornOS website: dovetail joints and brass inlay.

Subject: Fine cabinetmaking: exposed dovetail and finger joints at the corners, and thin brass inlay lines running straight and parallel along each side, turning at right angles only in the corners. The joints are the detail; there is no carving.

Material: oiled walnut, oak and dark bronze.

Light: Warm amber light grazes the inner left and lower edges of the opening. The outer edge of the surround also catches a thin sand-coloured highlight.

Feel: small and strict: the fewest parts that do the job. Plain, exact joinery rather than ornament; weight and warmth without decoration for its own sake. The page around and inside it is a very dark brown (#1F1711), so the surround must be clearly lighter than that: mid-tone wood and metal with lit edges, never a near-black frame that disappears into the page.

Colour: a narrow range of warm browns from deep umber through acorn brown (#644532) to pale sand (#E4CBA6) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else, and no green.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain very dark warm brown (#1F1711), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
green, pine green, moss, ferns, stone, marble, malachite, blue, purple, grey, near-black frame, black frame, charcoal, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 4. AcornOS, mineral frame: end-grain oak rings

Used for: Beside the "Updates you can undo" text on the homepage. Save as `acorn-frame-mineral.png`.

Positive:

```text
Frame artwork for the AcornOS website: end-grain oak rings.

Subject: End-grain oak: growth rings and medullary rays, the grain running straight and parallel along each side, with the tight concentric rings of the tree's heart showing only in the corners. A few hairline checks filled with dark bronze.

Material: oiled walnut, oak and dark bronze.

Light: Warm amber light grazes the inner left and lower edges of the opening. The outer edge of the surround also catches a thin sand-coloured highlight.

Feel: small and strict: the fewest parts that do the job. Plain, exact joinery rather than ornament; weight and warmth without decoration for its own sake. The page around and inside it is a very dark brown (#1F1711), so the surround must be clearly lighter than that: mid-tone wood and metal with lit edges, never a near-black frame that disappears into the page.

Colour: a narrow range of warm browns from deep umber through acorn brown (#644532) to pale sand (#E4CBA6) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else, and no green.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain very dark warm brown (#1F1711), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
green, pine green, moss, ferns, stone, marble, malachite, blue, purple, grey, near-black frame, black frame, charcoal, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 5. AcornOS, paper frame: folded kraft paper

Used for: The opening of the documentation's Getting Started page. Save as `acorn-frame-paper.png`.

Positive:

```text
Frame artwork for the AcornOS website: folded kraft paper.

Subject: Layers of heavy brown kraft paper and card, cut and folded: stacked sheets stepping down toward the opening, their cut edges running straight along each side, with sharp folded and overlapped corners like an envelope or a folded map.

Material: oiled walnut, oak and dark bronze.

Light: Warm amber light grazes the inner left and lower edges of the opening. The outer edge of the surround also catches a thin sand-coloured highlight.

Feel: small and strict: the fewest parts that do the job. Plain, exact joinery rather than ornament; weight and warmth without decoration for its own sake. The page around and inside it is a very dark brown (#1F1711), so the surround must be clearly lighter than that: mid-tone wood and metal with lit edges, never a near-black frame that disappears into the page.

Colour: a narrow range of warm browns from deep umber through acorn brown (#644532) to pale sand (#E4CBA6) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else, and no green.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain very dark warm brown (#1F1711), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
green, pine green, moss, ferns, stone, marble, malachite, blue, purple, grey, near-black frame, black frame, charcoal, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 6. AcornOS, atlas frame: contour map in leather

Used for: The Downloads page. Save as `acorn-frame-atlas.png`.

Positive:

```text
Frame artwork for the AcornOS website: contour map in leather.

Subject: A contour map tooled into tan leather laid over wood: contour lines running nearly parallel along each side and bunching into hills and a few spot heights (small dots, no numbers) only in the corners. A row of small bronze tacks sits just inside the outer edge at the corners only.

Material: oiled walnut, oak and dark bronze.

Light: Warm amber light grazes the inner left and lower edges of the opening. The outer edge of the surround also catches a thin sand-coloured highlight.

Feel: small and strict: the fewest parts that do the job. Plain, exact joinery rather than ornament; weight and warmth without decoration for its own sake. The page around and inside it is a very dark brown (#1F1711), so the surround must be clearly lighter than that: mid-tone wood and metal with lit edges, never a near-black frame that disappears into the page.

Colour: a narrow range of warm browns from deep umber through acorn brown (#644532) to pale sand (#E4CBA6) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else, and no green.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain very dark warm brown (#1F1711), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
green, pine green, moss, ferns, stone, marble, malachite, blue, purple, grey, near-black frame, black frame, charcoal, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 7. AcornOS, link preview background

Used for: the picture shown when a AcornOS page is shared in a chat or on social media. Save as `acorn-og.png`.

Positive:

```text
Background artwork for the AcornOS link preview image: a wide version of its hero frame.

Subject: An architectural aperture. Large, restrained oak-leaf vein relief in the upper-left and lower-right corners, as if a single huge oak leaf were pressed into the wood. In one other corner the cross-hatched scale pattern of an acorn cap, cut shallow into bronze. One small lacquer-red inlay interrupts the material in the lower-right corner.

Material: oiled walnut, oak and dark bronze.

Light: Warm amber light grazes the inner left and lower edges of the opening. The outer edge of the surround also catches a thin sand-coloured highlight.

Feel: small and strict: the fewest parts that do the job. Plain, exact joinery rather than ornament; weight and warmth without decoration for its own sake. The page around and inside it is a very dark brown (#1F1711), so the surround must be clearly lighter than that: mid-tone wood and metal with lit edges, never a near-black frame that disappears into the page.

Colour: a narrow range of warm browns from deep umber through acorn brown (#644532) to pale sand (#E4CBA6) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else, and no green.

Format: a wide landscape image, exactly 1536 x 806 (it is cropped to 1200 x 630 for link previews). A flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, seen head-on, filling the image to all four edges. The surround is about 110 px thick at top and bottom and about 130 px at left and right; its richest detail is in the corners. The opening is one flat, even, plain very dark warm brown (#1F1711) with nothing in it: the wordmark and one line of text are added there afterwards, so it must stay completely empty and evenly coloured.

No text, no logo, no controls anywhere.
```

Negative:

```text
green, pine green, moss, ferns, stone, marble, malachite, blue, purple, grey, near-black frame, black frame, charcoal, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

## LevitateOS

### 8. LevitateOS, hero frame: leaf-vein relief in stone

Used for: The homepage hero, the largest and most seen. It sets the family look. Save as `levitate-frame-hero.png`.

Positive:

```text
Frame artwork for the LevitateOS website: leaf-vein relief in stone.

Subject: An architectural aperture. Large, restrained botanical leaf-vein relief in the upper-left and lower-right corners, as if a single huge leaf were pressed into the stone. One small lacquer-red inlay interrupts the material in the lower-right corner.

Material: dark pine-green stone and blackened green metal.

Light: Warm amber light grazes the inner left and lower edges of the opening, as if the page inside were lit from the upper right.

Feel: intimidating competence expressed through warmth, weight and exactness. Substantial material planes, precise asymmetric bevels, deep shadow cuts. The page it surrounds is light paper, so the surround is dark and heavy against it.

Colour: a narrow range of greens from near-black pine through pine (#214438) to pale sage (#C7D9BA) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain pale sage (#DDE2D2), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
brown wood, oak, walnut, bronze, acorns, autumn colours, blue, purple, bright green, lime green, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 9. LevitateOS, fern frame: pressed fern fronds

Used for: The homepage's "Nothing hidden" introduction. Save as `levitate-frame-fern.png`.

Positive:

```text
Frame artwork for the LevitateOS website: pressed fern fronds.

Subject: Herbarium relief: fern fronds pressed into the surface, curling out of the corners and lying along the sides only as fine, even, parallel ribbing. The fronds are in the corners; the sides carry only the ribbing.

Material: dark pine-green stone and blackened green metal.

Light: Warm amber light grazes the inner left and lower edges of the opening, as if the page inside were lit from the upper right.

Feel: intimidating competence expressed through warmth, weight and exactness. Substantial material planes, precise asymmetric bevels, deep shadow cuts. The page it surrounds is light paper, so the surround is dark and heavy against it.

Colour: a narrow range of greens from near-black pine through pine (#214438) to pale sage (#C7D9BA) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain pale sage (#DDE2D2), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
brown wood, oak, walnut, bronze, acorns, autumn colours, blue, purple, bright green, lime green, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 10. LevitateOS, circuit frame: engraved circuit traces

Used for: Around the recipe code sample on the homepage. Save as `levitate-frame-circuit.png`.

Positive:

```text
Frame artwork for the LevitateOS website: engraved circuit traces.

Subject: Engraved traces like a circuit board, cut into patinated green metal: fine parallel lines running straight along each side, turning at right angles and ending in small round pads only in the corners.

Material: dark pine-green stone and blackened green metal.

Light: Warm amber light grazes the inner left and lower edges of the opening, as if the page inside were lit from the upper right.

Feel: intimidating competence expressed through warmth, weight and exactness. Substantial material planes, precise asymmetric bevels, deep shadow cuts. The page it surrounds is light paper, so the surround is dark and heavy against it.

Colour: a narrow range of greens from near-black pine through pine (#214438) to pale sage (#C7D9BA) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain pale sage (#DDE2D2), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
brown wood, oak, walnut, bronze, acorns, autumn colours, blue, purple, bright green, lime green, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 11. LevitateOS, mineral frame: banded green stone

Used for: Beside the "Updates you can undo" text on the homepage. Save as `levitate-frame-mineral.png`.

Positive:

```text
Frame artwork for the LevitateOS website: banded green stone.

Subject: Cut and polished green stone: banded layers like malachite or serpentine, the bands running parallel along each side and folding into tighter curves only in the corners. A few hairline fractures filled with dark metal.

Material: dark pine-green stone and blackened green metal.

Light: Warm amber light grazes the inner left and lower edges of the opening, as if the page inside were lit from the upper right.

Feel: intimidating competence expressed through warmth, weight and exactness. Substantial material planes, precise asymmetric bevels, deep shadow cuts. The page it surrounds is light paper, so the surround is dark and heavy against it.

Colour: a narrow range of greens from near-black pine through pine (#214438) to pale sage (#C7D9BA) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain pale sage (#DDE2D2), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
brown wood, oak, walnut, bronze, acorns, autumn colours, blue, purple, bright green, lime green, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 12. LevitateOS, paper frame: folded green paper

Used for: The opening of the documentation's Getting Started page. Save as `levitate-frame-paper.png`.

Positive:

```text
Frame artwork for the LevitateOS website: folded green paper.

Subject: Layers of heavy dark green paper and card, cut and folded: stacked sheets stepping down toward the opening, their cut edges running straight along each side, with sharp folded and overlapped corners like an envelope or a folded map.

Material: dark pine-green stone and blackened green metal.

Light: Warm amber light grazes the inner left and lower edges of the opening, as if the page inside were lit from the upper right.

Feel: intimidating competence expressed through warmth, weight and exactness. Substantial material planes, precise asymmetric bevels, deep shadow cuts. The page it surrounds is light paper, so the surround is dark and heavy against it.

Colour: a narrow range of greens from near-black pine through pine (#214438) to pale sage (#C7D9BA) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain pale sage (#DDE2D2), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
brown wood, oak, walnut, bronze, acorns, autumn colours, blue, purple, bright green, lime green, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 13. LevitateOS, atlas frame: contour map in metal

Used for: The Downloads page. Save as `levitate-frame-atlas.png`.

Positive:

```text
Frame artwork for the LevitateOS website: contour map in metal.

Subject: A contour map engraved into dark green metal: contour lines running nearly parallel along each side and bunching into hills and a few spot heights (small dots, no numbers) only in the corners.

Material: dark pine-green stone and blackened green metal.

Light: Warm amber light grazes the inner left and lower edges of the opening, as if the page inside were lit from the upper right.

Feel: intimidating competence expressed through warmth, weight and exactness. Substantial material planes, precise asymmetric bevels, deep shadow cuts. The page it surrounds is light paper, so the surround is dark and heavy against it.

Colour: a narrow range of greens from near-black pine through pine (#214438) to pale sage (#C7D9BA) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else.

Format: a 3:2 landscape image, exactly 1536 x 1024. It is a flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, like a deep window reveal seen head-on. No perspective, no tilt, no wall or room around it: the surround fills the image to all four edges.

Only the outer ring of this image is used on the page: a band about 154 px wide along all four edges. The browser cuts that band into four corner squares and four sides, keeps the corners as they are and stretches the sides to fit. Everything inside the band is thrown away. So:
- The material fills the whole band on every side, with the lit inner lip (where the surround steps down to the opening) about 130 to 150 px in from each edge.
- The four corner squares carry the richest detail.
- Along each side, between the corners, the material runs uniform lengthwise: straight lines, grain or layers parallel to the edge, with no single leaf, knot, object or spot there (it would be smeared when stretched).
- The opening is one flat, even, plain pale sage (#DDE2D2), with nothing in it. A live web page panel covers it.

No text, no logo, no controls anywhere.
```

Negative:

```text
brown wood, oak, walnut, bronze, acorns, autumn colours, blue, purple, bright green, lime green, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

### 14. LevitateOS, link preview background

Used for: the picture shown when a LevitateOS page is shared in a chat or on social media. Save as `levitate-og.png`.

Positive:

```text
Background artwork for the LevitateOS link preview image: a wide version of its hero frame.

Subject: An architectural aperture. Large, restrained botanical leaf-vein relief in the upper-left and lower-right corners, as if a single huge leaf were pressed into the stone. One small lacquer-red inlay interrupts the material in the lower-right corner.

Material: dark pine-green stone and blackened green metal.

Light: Warm amber light grazes the inner left and lower edges of the opening, as if the page inside were lit from the upper right.

Feel: intimidating competence expressed through warmth, weight and exactness. Substantial material planes, precise asymmetric bevels, deep shadow cuts. The page it surrounds is light paper, so the surround is dark and heavy against it.

Colour: a narrow range of greens from near-black pine through pine (#214438) to pale sage (#C7D9BA) on the lit edges, plus warm amber light and one small lacquer red (#B22F27) detail. Nothing else.

Format: a wide landscape image, exactly 1536 x 806 (it is cropped to 1200 x 630 for link previews). A flat, straight-on, front-facing rectangular surround with a plain rectangular opening in the middle, seen head-on, filling the image to all four edges. The surround is about 110 px thick at top and bottom and about 130 px at left and right; its richest detail is in the corners. The opening is one flat, even, plain pale sage (#DDE2D2) with nothing in it: the wordmark and one line of text are added there afterwards, so it must stay completely empty and evenly coloured.

No text, no logo, no controls anywhere.
```

Negative:

```text
brown wood, oak, walnut, bronze, acorns, autumn colours, blue, purple, bright green, lime green, text, letters, words, numbers, logo, watermark, signature, user interface, buttons, people, animals, picture inside the frame, scene inside the opening, scenery, outdoor scene, objects in the opening, texture in the opening, gradient in the opening, perspective view, tilted, angled, rotated, three-quarter view, wall, room, floor, table, drop shadow outside the frame, outer margin, white border, rounded opening, oval opening, arch, thin frame, ornate baroque moulding, gilded picture frame, gold leaf, cherubs, scrollwork, photo of a framed picture, symmetrical ornament on the sides, single objects on the sides, multicoloured, rainbow colours, neon, oversaturated, candy colours, plastic look, 3d cartoon, low detail, blurry, smeared texture, noise, jpeg artifacts
```

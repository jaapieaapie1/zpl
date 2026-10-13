# Spec decisions: where we follow the ZPL spec and where Labelary

The ZPL II Programming Guide in `docs/` is the source of truth. Labelary is the reference
renderer the compat harness scores against, but it is only a tiebreaker where the guide is
silent or contradicts itself. Every behaviour below was once justified by a Labelary render;
each row records what the guide says and what we do.

Class: **a** the spec agrees with Labelary, kept. **b** the spec is silent or ambiguous,
Labelary kept as tiebreaker. **c** the spec says otherwise: we follow the spec, and the compat
cases showing it are tagged `spec-deviates:` (their baseline is our spec-correct render).
**o** not a Labelary behaviour, but we contradicted the spec and now follow it.

Paths are relative to `docs/commands/`.

## Units and page

| Behaviour | Spec reference | Labelary | Ours | Class, reason |
|---|---|---|---|---|
| ^MU inches | `zpl/MU.md` example: "Assume 8 dot/millimeter (203 dot/inch) printer", `^MUi^FO.493` is `^FO100`; `zpl/LH.md`: 203 dots = 1 inch | 200 dots per inch at 8 dots/mm | the nominal 152 / 203 / 300 / 600 dpi | c: the example only works at 203. Cases: `mu_inches` |
| Nominal dpi at 6 dots/mm | `zpl/LH.md`: 152; `zpl/GB.md`, `zpl/JM.md`: 153 | 152 | 152 | b: the guide contradicts itself; 25.4 x 6 rounds to 152 |
| ^MU also converts the ^BY module width | `zpl/BY.md`: w "module width (in dots)"; `zpl/MU.md`: units "the printer uses" | converts | converts | b: which lengths ^MU covers is not listed |
| Negative ^FO / ^FT coordinates | `zpl/FO.md`, `zpl/FT.md`: values 0 to 32000 | minus sign ignored | minus sign ignored | b: out-of-range handling is not specified |
| ^PW narrower than the media is centred; the page is as tall as the media | `zpl/PW.md`, `zpl/LL.md`: silent on placement | centred | centred, in the compat harness only (`place_on_media`) | b: harness only, never reaches the renderers |

## Fonts and text

| Behaviour | Spec reference | Labelary | Ours | Class, reason |
|---|---|---|---|---|
| ^A applies to one field; omitted sizes come from ^CF | `zpl/A.md`: "used only once for that ^FD entry"; h, w "Default: last accepted ^CF" | same | same | a |
| ^CF without sizes | `zpl/CF.md`: "If neither value is defined, the last ^CF values given or the default ^CF values ... are used" | back to the power-up size | keeps the last ^CF sizes | c. Cases: `cf_default_font` |
| ^A orientation defaults to the ^FW in force | `zpl/A.md`: "Default: the last accepted ^FW value" | same | same | a |
| `^AF,N,13,26`: the comma after the font leaves the orientation empty | `zpl/A.md`: format `^Afo,h,w` | N lands in the height slot, ignored | same | a for the positions; b for a non-numeric size counting as not given |
| A width without a height magnifies bitmap fonts by the width's factor | `zpl/A.md`: "If you specify only the height or width value, the standard matrix for that font automatically determines the other value"; `zpl/CF.md`: "forces the magnification to be proportional" | same | same | a |
| Font 0 with only one size is as wide as tall | `zpl/A.md` (as above); `other/FONTS.md`: font 0 "DEFAULT: 15 x 12", "setting the height and width equally produces characters that appear the most balanced" | equal | equal | b: whether font 0's 15 x 12 default is a "standard matrix" ratio is unclear. Open question |
| Font 0 at least 10 dots each way | `zpl/A.md`: scalable h, w "10 to 32000" | same | same | a |
| Bitmap font faces: E is OCR-B, H is OCR-A | `other/FONTS.md` Table 29 type column | OCR-B / OCR-A outlines | same faces | a |
| Font E and H pitch and baseline at 6, 12 and 24 dots/mm | `other/FONTS.md` Tables 30-33 give the matrices (which we use); Table 29 gives gap and baseline only for the 8 dots/mm matrices | own values | Labelary's | b: undocumented |
| Font P width | `other/FONTS.md` Table 31: P is 20 x 18 | 17 dots wide | 18 | c. Cases: `font_p`, `fonts_overview` |
| Fonts P-V glyphs drawn at about 82% x 94.5% of the matrix | `other/FONTS.md`: matrices only | smaller glyphs | same | b |
| Characters a bitmap font lacks print blank | `other/FONTS.md`: "Unidentified characters should default to a space" | blank | blank | a |
| Font H prints lowercase blank | `other/FONTS.md`: H type "OCR-A" (not "U") | blank | blank | b: the repertoire is not listed; blank is the documented fallback |
| Font B prints lowercase as capitals | `other/FONTS.md`: B type "U" (uppercase) | capitals | capitals | b: whether lowercase is "unidentified" (a space) is not said. Open question |
| Font 0 hyphen drawn as its wide minus; Greek/Cyrillic fall back to condensed DejaVu | silent | as described | same | b |
| ^GS: the first character picks the symbol, in a 24-dot cell | `zpl/GS.md`: "^FD and the appropriate character"; `other/FONTS.md`: GS 24 x 24 | same | same | a for the cell; b for ignoring further characters and for the ink boxes |
| ^FB: block as tall as its maximum lines | `zpl/FB.md`: "^FT uses the baseline origin of the last possible line of text" | same | same | a |
| ^FB: space at a line break when centring / right-justifying; last line centred as if a space followed | `zpl/FB.md`: "word spaces at line breaks are discarded" | keeps room for the space | the space does not count | c. Cases: `fb_center`, `fb_right`, `fb_bitmap_font`, `fb_rotated`, `fb_newlines`, `fb_ft` |
| ^FB: a word longer than the line | `zpl/FB.md`: "a hyphen is automatically placed in the word at the right edge of the block. The remainder of the word is on the next line" | split without a hyphen | split with a hyphen (^FB only; ^TB and ^PA text without) | o |
| ^FB narrower than the font, or width 0 | `zpl/FB.md`: a "If the value is less than the font width or not specified, the text does not print" | prints | prints nothing | o |
| ^FB `\-` prints as a dash | `zpl/FB.md`: "soft hyphen characters are always printed as if they were a hyphen" (an earlier line says they are ignored away from a line end) | dash | dash | a, following the later, unconditional sentence |
| ^TB line pitch about 1.26 line heights for font 0 | `zpl/TB.md`: silent | 1.26 | 1.26 | b |
| ^FPV inter-character gap | `zpl/FP.md`: g "additional inter-character gap", no direction excluded | ignored | between the stacked characters | c. Cases: `fp_vertical` |
| ^FPR: the line ends an 'M' minus the gap past the origin, moved back only left or up on the page | `other/FIELD.md`: figures only, not in the text | as described | same | b |
| ^FO puts the rotated text box's top-left on the origin; ^FT the first baseline's start | `zpl/FO.md`: "upper-left corner of the field area ... independent of the rotation"; `zpl/FT.md` | same | same | a |
| Auto justification is left | `zpl/FW.md`: "auto justification (script dependent)" | left | left (left-to-right text) | a |

## Graphics

| Behaviour | Spec reference | Labelary | Ours | Class, reason |
|---|---|---|---|---|
| ^GD orientation `\` | `zpl/GD.md`: "R (or /) = right-leaning diagonal", "L (or \\) = left-leaning diagonal" | `\` leans right | `\` leans left, like L | c. Cases: `gd_diagonals` |
| ^GD thickness horizontal on every row, sticking out on the right | `zpl/GD.md`: "border thickness" only | as described | same | b |
| ^GB corner radius | `zpl/GB.md`: "rounding-radius = (rounding-index / 8) * (shorter side / 2)" | same | same | a |
| ^GB never smaller than its thickness | `zpl/GB.md`: w, h "value of t to 32000" | same | same | a |
| ^FO / ^FT z=1 right-justifies graphics and symbols too | `zpl/FO.md`, `zpl/FT.md`: z documented for fields generally | same | same | b |

## 1D barcodes

| Behaviour | Spec reference | Labelary | Ours | Class, reason |
|---|---|---|---|---|
| ^BY wide bar rounded down | `zpl/BY.md` Table 6 (e.g. w 3, r 2.5 is 2.3:1, i.e. 7 dots); the prose example rounds 21.6 to 22 | floor | floor | a: the table is consistently floor; the example contradicts it |
| ^FT origin of a barcode | `zpl/FT.md` Table 7: "the base of barcode, at the left edge" | same | same | a |
| Interpretation line gaps (6 dots below, 8 + 2 modules above), size growing with the module up to 10 | silent | as described | same | b |
| Interpretation line in the font of a preceding ^A | `zpl/BC.md` f: "can be printed in any font by placing the font command before the barcode command" | not checked | not implemented | open: a spec feature we lack |
| EAN/UPC digit layout (13-dot guard extensions, size steps, one centred line above) | silent | as described | same | b |
| EAN/UPC interpretation font | `zpl/BU.md`: at 8 dots/mm "a modulus of 3 dots or greater prints with an OCR-B interpretation line; a modulus of 1 or 2 dots prints font A" (2 / 5 / 9 at 6 / 12 / 24 dots/mm) | OCR-B and font A as documented | was DejaVu Sans Mono (font A's face) at every module; now as documented | o: Labelary agreed with the spec, we did not |
| Code 128 mode D: one FNC1, AIs packed in subset C, larger interpretation line | `zpl/BC.md`: mode D behaviour, no sizes | as described | same | b |
| Postal bar pitch `floor(2.5 * module)`, bar heights in thirds, interpretation line | `zpl/BZ.md`: "supports a print ratio of 2.0:1 to 3.0:1", geometry deferred to USPS | as described | same | b |
| Plessey stop bar; ^BJ start / stop bars | `zpl/BP.md`, `zpl/BJ.md`: patterns not given | as described | same | b |

## 2D symbols

| Behaviour | Spec reference | Labelary | Ours | Class, reason |
|---|---|---|---|---|
| ^B7 row height h | `zpl/B7.md` h: "This number multiplied by the module equals the height of the individual rows in dots" (the heading says "in dots"; ^BB uses the same wording and we multiply there too) | h dots | h x module dots | c. Cases: `pdf417_*`, `symbol_ft_orientations`, `fo_right_symbols`, `ft_barcodes`, `bq_module_width` |
| ^B7 default security level | `zpl/B7.md` s: "Default: 0 (error detection only)" | 0 | 0 | a |
| ^B7 grid without rows or columns | `zpl/B7.md`: "Default: 1:2 (row-to-column aspect ratio)", but the example puts 72 code words in 6 columns and 12 rows (2:1) | height:width of about 1:2 at 3-module rows | Labelary's | b: the rule and the example disagree, Labelary matches neither literally. Open question |
| PDF417 / MicroPDF417 compaction choices | silent | as described | same | b |
| ^BQ mask | `zpl/BQ.md` e: "mask value 0 - 7, Default: 7" | ignored (lowest penalty) | the given mask, else 7 | c. Cases: `qr_*`, `bq_module_width`, `fo_right_symbols`, `ft_barcodes` |
| ^BQ position | `zpl/FO.md`: the upper-left corner of the field; `zpl/FT.md` Table 7: a bar code's origin is its base at the left edge; `zpl/BQ.md`: "^FW has no effect on rotation" | ^FO: 10 dots down (right for B); ^FT: base 3 modules above the origin | ^FO: top-left on the origin; ^FT: base on the origin | c: same cases. Labelary's offset may be real printer behaviour; see open questions |
| ^BQ sets the ^BY module width of later barcodes | `zpl/BY.md`: "it stays in effect until another ^BY command is encountered" | sets it | leaves it | c. Cases: `bq_module_width` |
| ^BQ default magnification by resolution | `zpl/BQ.md` c: 1 / 2 / 3 / 6 at 150 / 200 / 300 / 600 dpi | same | same | a |
| ^BQ error correction raised within the version | `zpl/BQ.md`: silent | raised | raised | b (also a Zebra printer behaviour, kept by earlier decision) |
| ^BQ automatic input as one segment; manual N / A keeps only the mode's characters | `zpl/BQ.md`: silent on segmentation and invalid characters | as described | same | b |
| ^BX requested size never grown | `zpl/BX.md`: "If you attempt to force the data into too small of a symbol, no symbol is printed" | same | same | a |
| ^BX encodation (ISO/IEC 16022 annex P look-ahead) | `zpl/BX.md`: silent; ISO/IEC 16022 | same | same | b: the symbology standard |
| ^B0 / ^BO sizing and mode latches (zint) | silent | as described | same | b |
| ^BD MaxiCode printed at 25 x 24 mm (200 x 192 dots at 8 dots/mm), 0.75 mm hexagons | `zpl/BD.md`: silent on size; `zpl/JM.md`: "If ^JMB is used, the UPS MaxiCode bar code becomes out of specification"; ISO/IEC 16023 nominal 28.14 x 26.91 mm | 25 x 24 mm | Labelary's | b: the guide gives no size. Open question |
| ^FH bytes reach 2D encoders as raw bytes; ^CI does not re-encode 2D data | `zpl/FH.md`: silent for 2D symbols | as described | same | b |

## Field data

| Behaviour | Spec reference | Labelary | Ours | Class, reason |
|---|---|---|---|---|
| A `^` inside ^FD | `zpl/FD.md`: field data is "any printable character except those used as command prefixes (^ and ~)" | prints the data before it | prints the data before it (the caret starts the next command) | a |
| ^SN without leading zeros keeps its width with spaces (`007` prints `  7`) | `zpl/SN.md`: z=N suppresses leading zeros; the value includes "any leading spaces" | as described | same | b: consistent with the spec, not stated |
| ^SF leaves characters invalid for their mask position alone, stopping the carry | `zpl/SF.md`: only invalid increment characters are defined (as zero) | as described | same | b |

## Open questions

Decisions above that a printer, not Labelary, should settle:

- ^BQ: the spec places the symbol on the origin and defaults the mask to 7; Labelary (and
  reportedly Zebra printers) add a 10-dot band above ^FO symbols and pick the mask. We follow
  the spec.
- ^B7 h: the spec multiplies by the module width (followed); the heading says "in dots".
- ^B7 grid without rows or columns: the 1:2 rule and the 72-code-word example disagree.
- Font 0 with only a height: as wide as tall (Labelary) or 12/15 of it (the default matrix).
- Font B lowercase: capitals (Labelary) or spaces ("unidentified characters").
- MaxiCode size: Labelary's 25 x 24 mm or the ISO/IEC 16023 nominal 28.14 x 26.91 mm.
- Nominal dpi at 6 dots/mm: 152 (`LH.md`) or 153 (`GB.md`, `JM.md`).
- ^BC interpretation line in the font of a preceding ^A: documented, not implemented.

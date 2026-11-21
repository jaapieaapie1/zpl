#!/usr/bin/env python3
"""Builds ZplSansCn-Bold.otf, the scalable font 0, from TeX Gyre Heros Cn Bold.

ZPL font 0 is a Helvetica-style bold condensed face. TeX Gyre Heros Cn Bold has the
same design under the GUST Font License. Zebra sizes font 0 in em with capitals 0.75 em
tall, and lines of text only line up with printed labels when every character advances
as far as on the printer. This script therefore:

- scales the outlines so the cap height is 0.75 em (Heros: 0.718 em),
- sets the advance width of each character to the font 0 metrics below, centring the
  glyph in its new width (other glyphs keep their scaled width),
- fits the ASCII glyphs, and the accented letters built on them, to the ink of font 0
  below: font 0's characters are narrower than Heros' with heavier stems and lighter
  bars, so a glyph is scaled into its box and its strokes are then reweighted until it
  covers the same area,
- sets the vertical metrics below, drops the kerning (made for the old spacing),
- renames the font, as the GUST Font License asks of modified versions.

Usage (needs fonttools and skia-pathops):
    curl -LO https://mirrors.ctan.org/fonts/tex-gyre/opentype/texgyreheroscn-bold.otf
    python3 build_zpl_sans.py texgyreheroscn-bold.otf ZplSansCn-Bold.otf
"""
import math
import sys
import unicodedata

from fontTools.pens.boundsPen import BoundsPen
from fontTools.pens.qu2cuPen import Qu2CuPen
from fontTools.pens.t2CharStringPen import T2CharStringPen
from fontTools.pens.transformPen import TransformPen
from fontTools.ttLib import TTFont
from pathops import Path, PathOp, op, union

CAP_HEIGHT_SCALE = 750 / 718
# Font 0 prints slightly smaller than its nominal metrics: text set in it runs 0.9944 times
# as wide as the advance widths below add up to, and its capitals are that much lower.
FONT_0_SCALE = 0.9944

# Font 0 metrics in 1/1000 em: advance width per code point, and line metrics.
VERTICAL = {'ascent': 1053, 'descent': -294, 'typo_ascender': 750, 'typo_descender': -250, 'win_ascent': 1053, 'win_descent': 294}
ADVANCES = {
    0x0000: 0, 0x000D: 296, 0x0020: 296, 0x0021: 296, 0x0022: 481, 0x0023: 481, 0x0024: 481,
    0x0025: 907, 0x0026: 611, 0x0027: 296, 0x0028: 296, 0x0029: 296, 0x002A: 481, 0x002B: 907,
    0x002C: 296, 0x002D: 296, 0x002E: 296, 0x002F: 296, 0x0030: 481, 0x0031: 481, 0x0032: 481,
    0x0033: 481, 0x0034: 481, 0x0035: 481, 0x0036: 481, 0x0037: 481, 0x0038: 481, 0x0039: 481,
    0x003A: 296, 0x003B: 296, 0x003C: 1000, 0x003D: 907, 0x003E: 1000, 0x003F: 444, 0x0040: 907,
    0x0041: 556, 0x0042: 556, 0x0043: 537, 0x0044: 593, 0x0045: 500, 0x0046: 500, 0x0047: 593,
    0x0048: 611, 0x0049: 278, 0x004A: 444, 0x004B: 556, 0x004C: 481, 0x004D: 759, 0x004E: 611,
    0x004F: 574, 0x0050: 556, 0x0051: 574, 0x0052: 593, 0x0053: 537, 0x0054: 500, 0x0055: 611,
    0x0056: 537, 0x0057: 815, 0x0058: 556, 0x0059: 556, 0x005A: 500, 0x005B: 296, 0x005C: 481,
    0x005D: 296, 0x005E: 500, 0x005F: 500, 0x0060: 296, 0x0061: 463, 0x0062: 500, 0x0063: 444,
    0x0064: 500, 0x0065: 481, 0x0066: 278, 0x0067: 500, 0x0068: 500, 0x0069: 259, 0x006A: 259,
    0x006B: 444, 0x006C: 259, 0x006D: 759, 0x006E: 500, 0x006F: 481, 0x0070: 500, 0x0071: 500,
    0x0072: 333, 0x0073: 426, 0x0074: 278, 0x0075: 500, 0x0076: 444, 0x0077: 667, 0x0078: 444,
    0x0079: 444, 0x007A: 389, 0x007B: 500, 0x007C: 500, 0x007D: 500, 0x007E: 1000, 0x00A0: 296,
    0x00A1: 296, 0x00A2: 481, 0x00A3: 481, 0x00A4: 481, 0x00A5: 481, 0x00A6: 148, 0x00A7: 481,
    0x00A8: 360, 0x00A9: 500, 0x00AA: 481, 0x00AB: 407, 0x00AC: 907, 0x00AD: 296, 0x00AE: 500,
    0x00AF: 360, 0x00B0: 481, 0x00B1: 907, 0x00B2: 370, 0x00B3: 370, 0x00B4: 360, 0x00B5: 500,
    0x00B6: 481, 0x00B7: 481, 0x00B8: 360, 0x00B9: 370, 0x00BA: 481, 0x00BB: 407, 0x00BC: 907,
    0x00BD: 907, 0x00BE: 907, 0x00BF: 444, 0x00C0: 556, 0x00C1: 556, 0x00C2: 556, 0x00C3: 556,
    0x00C4: 556, 0x00C5: 556, 0x00C6: 870, 0x00C7: 537, 0x00C8: 500, 0x00C9: 500, 0x00CA: 500,
    0x00CB: 500, 0x00CC: 278, 0x00CD: 278, 0x00CE: 278, 0x00CF: 278, 0x00D0: 593, 0x00D1: 611,
    0x00D2: 574, 0x00D3: 574, 0x00D4: 574, 0x00D5: 574, 0x00D6: 574, 0x00D7: 907, 0x00D8: 574,
    0x00D9: 611, 0x00DA: 611, 0x00DB: 611, 0x00DC: 611, 0x00DD: 556, 0x00DE: 556, 0x00DF: 519,
    0x00E0: 463, 0x00E1: 463, 0x00E2: 463, 0x00E3: 463, 0x00E4: 463, 0x00E5: 463, 0x00E6: 722,
    0x00E7: 444, 0x00E8: 481, 0x00E9: 481, 0x00EA: 481, 0x00EB: 481, 0x00EC: 259, 0x00ED: 259,
    0x00EE: 259, 0x00EF: 259, 0x00F0: 481, 0x00F1: 500, 0x00F2: 481, 0x00F3: 481, 0x00F4: 481,
    0x00F5: 481, 0x00F6: 481, 0x00F7: 907, 0x00F8: 481, 0x00F9: 500, 0x00FA: 500, 0x00FB: 500,
    0x00FC: 500, 0x00FD: 444, 0x00FE: 500, 0x00FF: 444, 0x0131: 259, 0x0141: 481, 0x0142: 259,
    0x0152: 815, 0x0153: 741, 0x0160: 537, 0x0161: 426, 0x0178: 556, 0x017D: 500, 0x017E: 389,
    0x0192: 481, 0x02C6: 360, 0x02C7: 360, 0x02C9: 360, 0x02D8: 360, 0x02D9: 360, 0x02DA: 360,
    0x02DB: 360, 0x02DC: 360, 0x02DD: 360, 0x0394: 587, 0x03A9: 641, 0x03BC: 500, 0x03C0: 524,
    0x2013: 481, 0x2014: 907, 0x2018: 296, 0x2019: 296, 0x201A: 296, 0x201C: 426, 0x201D: 426,
    0x201E: 426, 0x2020: 481, 0x2021: 481, 0x2022: 500, 0x2026: 907, 0x2030: 1278, 0x2039: 241,
    0x203A: 241, 0x2044: 378, 0x20AC: 481, 0x2113: 448, 0x2122: 611, 0x2126: 641, 0x212E: 804,
    0x2202: 505, 0x2206: 587, 0x220F: 642, 0x2211: 507, 0x2212: 907, 0x2215: 378, 0x2219: 481,
    0x221A: 536, 0x221E: 746, 0x222B: 350, 0x2248: 500, 0x2260: 500, 0x2264: 500, 0x2265: 500,
    0x25CA: 536, 0xFB01: 537, 0xFB02: 537,
}

# Font 0 ink in 1/1000 em: the box (left, right, bottom, top) from the glyph origin on the
# baseline, and the inked area in 1000 square units. Measured on 200-dot reference renders,
# so good to about 3 units. The hyphen prints as the minus sign, so that is the glyph
# measured for it.
INK = {
    0x0021: (78, 219, 0, 745, 88), 0x0022: (78, 397, 457, 745, 60), 0x0023: (8, 483, -20, 754, 162),
    0x0024: (28, 460, -95, 780, 197), 0x0025: (42, 875, -20, 752, 288), 0x0026: (42, 592, -20, 745, 240),
    0x0027: (91, 225, 462, 745, 28), 0x0028: (66, 294, -176, 742, 108), 0x0029: (7, 240, -176, 742, 108),
    0x002A: (77, 409, 444, 745, 53), 0x002B: (168, 744, -24, 607, 121), 0x002C: (88, 227, -150, 134, 29),
    0x002E: (93, 232, 0, 134, 19), 0x002F: (11, 295, -20, 745, 92), 0x0030: (39, 442, -20, 752, 213),
    0x0031: (72, 366, 0, 737, 115), 0x0032: (29, 445, 0, 752, 179), 0x0033: (30, 454, -20, 752, 188),
    0x0034: (25, 458, 0, 737, 171), 0x0035: (45, 459, -20, 739, 196), 0x0036: (38, 457, -20, 752, 209),
    0x0037: (30, 450, 0, 739, 136), 0x0038: (34, 452, -20, 752, 214), 0x0039: (33, 449, -20, 752, 207),
    0x003A: (84, 222, 0, 555, 38), 0x003B: (84, 227, -150, 555, 49), 0x003C: (204, 800, -11, 604, 106),
    0x003D: (168, 744, 132, 465, 127), 0x003E: (204, 800, -6, 609, 106), 0x003F: (37, 422, 0, 759, 129),
    0x0040: (153, 755, -125, 695, 291), 0x0041: (16, 550, 0, 745, 209), 0x0042: (74, 512, 0, 745, 252),
    0x0043: (47, 495, -20, 760, 195), 0x0044: (72, 549, 0, 745, 242), 0x0045: (69, 466, 0, 745, 193),
    0x0046: (68, 471, 0, 745, 168), 0x0047: (51, 534, -20, 760, 239), 0x0048: (72, 544, 0, 745, 232),
    0x0049: (69, 213, 0, 745, 107), 0x004A: (18, 394, -24, 745, 145), 0x004B: (68, 552, 0, 745, 227),
    0x004C: (69, 473, 0, 745, 138), 0x004D: (67, 693, 0, 745, 336), 0x004E: (71, 546, 0, 745, 258),
    0x004F: (46, 528, -20, 760, 236), 0x0050: (72, 534, 0, 745, 198), 0x0051: (46, 565, -69, 760, 250),
    0x0052: (72, 567, 0, 745, 252), 0x0053: (46, 497, -20, 760, 215), 0x0054: (25, 485, 0, 745, 144),
    0x0055: (68, 544, -20, 745, 226), 0x0056: (20, 527, 0, 745, 197), 0x0057: (22, 814, 0, 745, 325),
    0x0058: (14, 546, 0, 745, 204), 0x0059: (20, 551, 0, 745, 158), 0x005A: (39, 471, 0, 745, 181),
    0x005B: (50, 275, -175, 744, 148), 0x005C: (41, 450, -15, 720, 167), 0x005D: (26, 251, -175, 744, 145),
    0x005E: (59, 462, 477, 744, 40), 0x005F: (2, 498, -251, -203, 24), 0x0060: (76, 218, 459, 741, 28),
    0x0061: (38, 430, -15, 570, 162), 0x0062: (66, 458, -15, 745, 194), 0x0063: (43, 403, -15, 570, 137),
    0x0064: (48, 445, -15, 745, 195), 0x0065: (52, 435, -15, 570, 162), 0x0066: (25, 264, 0, 754, 115),
    0x0067: (48, 442, -192, 570, 214), 0x0068: (69, 446, 0, 745, 179), 0x0069: (65, 200, 0, 731, 90),
    0x006A: (23, 205, -194, 731, 115), 0x006B: (63, 450, 0, 745, 173), 0x006C: (65, 199, 0, 745, 100),
    0x006D: (68, 696, 0, 570, 243), 0x006E: (64, 446, 0, 570, 156), 0x006F: (48, 439, -15, 570, 160),
    0x0070: (67, 458, -186, 570, 193), 0x0071: (48, 447, -186, 570, 193), 0x0072: (69, 325, 0, 570, 89),
    0x0073: (36, 410, -15, 570, 144), 0x0074: (20, 261, -16, 706, 109), 0x0075: (65, 445, -15, 551, 156),
    0x0076: (10, 434, 0, 551, 127), 0x0077: (24, 649, 0, 551, 209), 0x0078: (11, 438, 0, 551, 133),
    0x0079: (17, 436, -194, 551, 155), 0x007A: (22, 372, 0, 551, 121), 0x007B: (82, 439, -235, 759, 141),
    0x007C: (232, 281, -250, 744, 49), 0x007D: (62, 414, -235, 759, 139), 0x007E: (206, 821, 216, 371, 54),
    0x2212: (168, 744, 236, 346, 63),
}
# Height of a horizontal bar in 1/1000 em: Heros at the 0.75 em cap height, and font 0.
BAR, FONT_0_BAR = 132, 118
# The widest stroke change tried, in 1/1000 em.
MAX_SWEEP = 60


def shape(draw, bounds, ink, sweep, trim):
    """The outline drawn by `draw(pen, transform)` with `bounds`, fitted in the box `ink`.

    The glyph is scaled, then swept `sweep` units sideways, which thickens its vertical
    strokes (a negative sweep thins them) without changing the horizontal ones, and `trim`
    units are taken off the top of every stroke, which thins the horizontal ones.
    """
    x0, y0, x1, y1 = bounds
    left, right, bottom, top = ink
    kx = (right - left - sweep) / (x1 - x0)
    ky = (top - bottom + trim) / (y1 - y0)
    # A thinned glyph loses its left edge to the intersection, so it starts further left.
    ox = left - x0 * kx + min(sweep, 0)
    oy = bottom - y0 * ky

    def copy(dx, dy):
        path = Path()
        draw(path.getPen(), (kx, 0, 0, ky, ox + dx, oy + dy))
        return path

    outline = Path()
    union([copy(0, 0)], outline.getPen())
    if sweep > 0:
        # Copies close enough together that their union is the swept outline.
        steps = 1 + math.ceil(sweep / 30)
        outline = Path()
        union([copy(sweep * i / steps, 0) for i in range(steps + 1)], outline.getPen())
    elif sweep < 0:
        outline = op(outline, copy(-sweep, 0), PathOp.INTERSECTION)
    if trim > 0:
        lowered = Path()
        outline.draw(TransformPen(lowered.getPen(), (1, 0, 0, 1, 0, -trim)))
        outline = op(outline, lowered, PathOp.INTERSECTION)
    return outline


def fit(draw, bounds, ink, letter):
    """(sweep, trim) fitting a glyph in `ink`; for letters and digits with font 0's weight.

    The bars of a letter are thinned to font 0's and its stems are then swept until the
    glyph covers the measured area. Other characters are only scaled into their box.
    """
    *box, area = ink
    height, target_height = bounds[3] - bounds[1], box[3] - box[2]
    if not letter:
        return 0.0, 0.0
    trim = 0.0
    if height > 2.2 * BAR:
        trim = (BAR * target_height / height - FONT_0_BAR) / (1 - BAR / height)
        trim = min(max(trim, 0.0), 0.3 * BAR)
    low, high = -MAX_SWEEP, MAX_SWEEP
    for _ in range(12):
        sweep = (low + high) / 2
        if abs(shape(draw, bounds, box, sweep, trim).area) < area * 1000:
            low = sweep
        else:
            high = sweep
    return (low + high) / 2, trim


def build(source, target):
    font = TTFont(source)
    cmap = font.getBestCmap()
    code_points = {}
    for cp, name in cmap.items():
        code_points.setdefault(name, cp)
    cff = font["CFF "].cff
    top = cff[cff.fontNames[0]]
    charstrings = top.CharStrings
    glyphs = font.getGlyphSet()
    s = CAP_HEIGHT_SCALE * FONT_0_SCALE
    advances = {cp: round(width * FONT_0_SCALE) for cp, width in ADVANCES.items()}

    def bounds_of(name, transform):
        pen = BoundsPen(glyphs)
        glyphs[name].draw(TransformPen(pen, transform))
        return pen.bounds

    def placement(name):
        advance, _ = font["hmtx"][name]
        new_advance = advances.get(code_points.get(name), round(advance * s))
        return new_advance, (s, 0, 0, s, (new_advance - advance * s) / 2, 0)

    def drawer(name, place):
        return lambda pen, transform: glyphs[name].draw(TransformPen(TransformPen(pen, transform), place))

    # Each measured character's box and strokes, also used for the accented letters built
    # on it: these get the base letter's scaling, sweep and trim.
    fits = {}
    for cp, ink in INK.items():
        name = cmap.get(cp)
        if name is None:
            continue
        _, place = placement(name)
        bounds = bounds_of(name, place)
        if bounds:
            sweep, trim = fit(drawer(name, place), bounds, ink, chr(cp).isalnum())
            fits[cp] = (bounds, ink[:4], sweep, trim)

    def fit_of(name):
        cp = code_points.get(name)
        if cp is None:
            return None
        if cp in fits:
            return fits[cp]
        base = unicodedata.decomposition(chr(cp)).split()
        if not base or base[0].startswith("<") or not unicodedata.category(chr(cp)).startswith("L"):
            return None
        base_cp = int(base[0], 16)
        same_width = base_cp in fits and ADVANCES.get(cp) == ADVANCES.get(base_cp)
        return fits[base_cp] if same_width else None

    rebuilt = {}
    for name in font.getGlyphOrder():
        new_advance, place = placement(name)
        pen = T2CharStringPen(new_advance, glyphs)
        fitted = fit_of(name)
        if fitted is None or bounds_of(name, place) is None:
            glyphs[name].draw(TransformPen(pen, place))
        else:
            outline = shape(drawer(name, place), *fitted)
            outline.draw(Qu2CuPen(pen, 0.5, all_cubic=True))
        old = charstrings[name]
        charstring = pen.getCharString(private=old.private, globalSubrs=old.globalSubrs)
        rebuilt[name] = (charstring, new_advance)
    for name, (charstring, advance) in rebuilt.items():
        charstrings[name] = charstring
    glyphs = font.getGlyphSet()
    for name, (_, advance) in rebuilt.items():
        bounds = bounds_of(name, (1, 0, 0, 1, 0, 0))
        font["hmtx"][name] = (advance, round(bounds[0]) if bounds else 0)

    hhea, os2 = font["hhea"], font["OS/2"]
    hhea.ascent, hhea.descent = VERTICAL["ascent"], VERTICAL["descent"]
    os2.sTypoAscender, os2.sTypoDescender = VERTICAL["typo_ascender"], VERTICAL["typo_descender"]
    os2.usWinAscent, os2.usWinDescent = VERTICAL["win_ascent"], VERTICAL["win_descent"]
    os2.sCapHeight, os2.sxHeight = round(os2.sCapHeight * s), round(os2.sxHeight * s)
    top.FontBBox = [round(v * s) for v in top.FontBBox]
    if "GPOS" in font:
        del font["GPOS"]

    for record in font["name"].names:
        text = record.toUnicode()
        for old, new in (("TeXGyreHerosCn", "ZplSansCn"), ("TeX Gyre Heros Cn", "ZPL Sans Cn"),
                         ("TeXGyreHeros", "ZplSans"), ("TeX Gyre Heros", "ZPL Sans")):
            text = text.replace(old, new)
        record.string = text
    cff.fontNames[0] = "ZplSansCn-Bold"
    top.FullName, top.FamilyName = "ZPL Sans Cn Bold", "ZPL Sans Cn"
    font.save(target)


if __name__ == "__main__":
    build(sys.argv[1], sys.argv[2])

#!/usr/bin/env python3
"""Builds ZplSansCn-Bold.otf, the scalable font 0, from TeX Gyre Heros Cn Bold.

ZPL font 0 is a Helvetica-style bold condensed face. TeX Gyre Heros Cn Bold has the
same design under the GUST Font License. Zebra sizes font 0 in em with capitals 0.75 em
tall, and lines of text only line up with printed labels when every character advances
as far as on the printer. This script therefore:

- scales the outlines so the cap height is 0.75 em (Heros: 0.718 em),
- sets the advance width of each character to the font 0 metrics below, centring the
  glyph in its new width (other glyphs keep their scaled width),
- sets the vertical metrics below, drops the kerning (made for the old spacing),
- renames the font, as the GUST Font License asks of modified versions.

Usage (needs fonttools):
    curl -LO https://mirrors.ctan.org/fonts/tex-gyre/opentype/texgyreheroscn-bold.otf
    python3 build_zpl_sans.py texgyreheroscn-bold.otf ZplSansCn-Bold.otf
"""
import sys

from fontTools.pens.t2CharStringPen import T2CharStringPen
from fontTools.pens.transformPen import TransformPen
from fontTools.ttLib import TTFont

CAP_HEIGHT_SCALE = 750 / 718

# Font 0 metrics in 1/1000 em: advance width per code point, and line metrics.
VERTICAL = {'ascent': 1053, 'descent': -294, 'typo_ascender': 750, 'typo_descender': -250, 'win_ascent': 1053, 'win_descent': 294}
ADVANCES = {
    0x0000: 0, 0x000D: 296, 0x0020: 296, 0x0021: 296, 0x0022: 426, 0x0023: 481, 0x0024: 481,
    0x0025: 907, 0x0026: 611, 0x0027: 296, 0x0028: 296, 0x0029: 296, 0x002A: 481, 0x002B: 907,
    0x002C: 296, 0x002D: 296, 0x002E: 296, 0x002F: 296, 0x0030: 481, 0x0031: 481, 0x0032: 481,
    0x0033: 481, 0x0034: 481, 0x0035: 481, 0x0036: 481, 0x0037: 481, 0x0038: 481, 0x0039: 481,
    0x003A: 296, 0x003B: 296, 0x003C: 907, 0x003D: 907, 0x003E: 907, 0x003F: 444, 0x0040: 800,
    0x0041: 556, 0x0042: 556, 0x0043: 537, 0x0044: 593, 0x0045: 500, 0x0046: 500, 0x0047: 593,
    0x0048: 611, 0x0049: 278, 0x004A: 444, 0x004B: 556, 0x004C: 481, 0x004D: 759, 0x004E: 611,
    0x004F: 574, 0x0050: 556, 0x0051: 574, 0x0052: 593, 0x0053: 537, 0x0054: 500, 0x0055: 611,
    0x0056: 537, 0x0057: 815, 0x0058: 556, 0x0059: 556, 0x005A: 500, 0x005B: 296, 0x005C: 296,
    0x005D: 296, 0x005E: 481, 0x005F: 500, 0x0060: 360, 0x0061: 463, 0x0062: 500, 0x0063: 444,
    0x0064: 500, 0x0065: 481, 0x0066: 278, 0x0067: 500, 0x0068: 500, 0x0069: 259, 0x006A: 259,
    0x006B: 444, 0x006C: 259, 0x006D: 759, 0x006E: 500, 0x006F: 481, 0x0070: 500, 0x0071: 500,
    0x0072: 333, 0x0073: 426, 0x0074: 278, 0x0075: 500, 0x0076: 444, 0x0077: 667, 0x0078: 444,
    0x0079: 444, 0x007A: 389, 0x007B: 296, 0x007C: 333, 0x007D: 296, 0x007E: 907, 0x00A0: 296,
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
    s = CAP_HEIGHT_SCALE

    rebuilt = {}
    for name in font.getGlyphOrder():
        advance, lsb = font["hmtx"][name]
        new_advance = ADVANCES.get(code_points.get(name), round(advance * s))
        dx = (new_advance - advance * s) / 2
        pen = T2CharStringPen(new_advance, glyphs)
        glyphs[name].draw(TransformPen(pen, (s, 0, 0, s, dx, 0)))
        old = charstrings[name]
        charstring = pen.getCharString(private=old.private, globalSubrs=old.globalSubrs)
        rebuilt[name] = (charstring, new_advance, round(lsb * s + dx))
    for name, (charstring, advance, lsb) in rebuilt.items():
        charstrings[name] = charstring
        font["hmtx"][name] = (advance, lsb)

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

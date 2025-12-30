# ^FL




ZPL Commands


The `^FL` command provides the ability to link any TrueType font, including private character fonts, to
associated fonts.


**Font Linking**


If the base font does not have a glyph for the required character, the printer looks to the linked fonts for
the glyph. The font links are user-definable. The font linking remains until the link is broken or the printer is
turned off. To permanently save the font linking, use the `^JUS` command.


**NOTE:** For assistance in setting up the font links, use the font wizard in ZebraNet Bridge.


**Format:** `^FL<ext>,<base>,<link>`

|Parameters|Details|
|---|---|
|`<ext>`|This is the fully-qualified filename of the extension. This file name does not<br>accept wildcards.<br>The supported extensions for this parameter are:`.TTF` and`.TTE`. The<br>format for this parameter is the memory device followed by the font name<br>with the extension, as follows:<br>`E:SWISS721.TTF`|
|`<base>`|This is the filename of the base font(s). The base font can be any of the<br>following types:<br>`.FNT`<br>`.TTF or`<br>`.TTE`<br>From these font types, you can only link to a`.TTF` or`TTE`.<br>The name of the base font can be expressed as a wild card; doing so<br>will define multiple base fonts. The result will be that all base font files so<br>defined will be linked to the file defined in the`<ext>` parameter.<br>The filename does not have to match a file that is currently defined on the<br>printer. A specification of`*.TTF` results in all *`.TTF` font files loaded on the<br>printer currently or in the future to be linked with the specified`<ext>` font<br>extension.|
|`<link>`|This is an indicator that determines if the extension is to be linked with the<br>base, or unlinked from the base, as follows:<br>**Values:**<br>`0 = <ext>` is to be unlinked (disassociated) from the file(s) specified in<br>`<base>`<br>`1 = <ext>` is to be linked (associated) with the file(s) specified by`<base>`<br>**Default:** must be an accepted value or it is ignored|



**Comments:** A font can have up to five fonts linked to it. The printer's resident font, `0.FNT` is always the last
font in the list of font links, but is not included in the five-link maximum. It can also be placed anywhere in
the font links list.


195


ZPL Commands


The default glyph prints when a glyph cannot be found in any of the fonts in the link list. The advanced
layout command `^PA` determines if the default glyph is a space character or the default glyph of the base
font, which is typically a hollow box.

The list of font links can be printed by using the `^LF` command or retrieved with the `^HT` command.

**Example:** These examples show the code and output for no font linking and for font linking:

```
No Font Linking

```

In the no-font linking example, the Swiss721 font does not have Asian glyphs, which is why Asian glyphs do
not print.

```
Font Linking

```

In the font linking example, this code is sent down to link the `ANMDJ.TTF` font to `SWISS721.TTF` font:

```
^XA

^FLE:ANMDJ.TTF,E:SWISS721.TTF,1^FS
^XZ

```

When the label prints, the Asian characters are printed using the `ANMDJ.TTF` font, rather than the
`SWISS721.TTF` font.


196
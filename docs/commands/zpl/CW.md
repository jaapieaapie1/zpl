# ^CW




ZPL Commands


All built-in fonts are referenced using a one-character identifier. The `^CW` command assigns a single
alphanumeric character to a font stored in DRAM, memory card, EPROM, or Flash.


**Font Identifier**


If the assigned character is the same as that of a built-in font, the downloaded font is used in place of the
built-in font. The new font is printed on the label wherever the format calls for the built-in font. If used in
place of a built-in font, the change is in effect only until power is turned off.


If the assigned character is different, the downloaded font is used as an additional font. The assignment
remains in effect until a new command is issued or the printer is turned off.

**Format:** `^CWa,d:o.x`






|Parameters|Details|
|---|---|
|`a =` letter of existing font<br>to be substituted, or new<br>font to be added|**Values:** `A` through`Z` and`0` to`9`<br>**Default:** a one-character entry is required|
|`d =` device to store font in<br>(optional)|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` name of the<br>downloaded font to be<br>substituted for the built-in,<br>or as an additional font|**Values:** any name up to 8 characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension<br>`.TTE` is only supported in<br>firmware version V60.14.x,<br>V50.14.x, or later.|**Values:**<br>`.FNT =` Font<br>`.TTF =` TrueType Font<br>`.TTE =` TrueType Extension|



**Example:** These examples show how to use:

- `MYFONT.FNT` stored in DRAM whenever a format calls for Font A:

```
^XA
^CWA,R:MYFONT.FNT
^XZ

```

- `MYFONT.FNT` stored in DRAM additionally as Font Q:

```
^XA
^CWQ,R:MYFONT.FNT
^XZ

```

- `NEWFONT.FNT` stored in DRAM whenever a format calls for font F:

```
^XA
^CWF,R:NEWFONT.FNT

```

168


ZPL Commands

```
^XZ

```

**Figure 14** Label Listing Before Assignment


**Figure 15** Label Listing After Assignment


169
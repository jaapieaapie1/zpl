# ^A@




ZPL Commands


The `^A@` command uses the complete name of a font, rather than the character designation used in `^A` .
Once a value for `^A@` is defined, it represents that font until a new font name is specified by `^A@` .


**Use Font Name to Call Font**


**Format:** ^A@o,h,w,d:f.x











|Parameter|Details|
|---|---|
|`o =` field orientation|**Values:**<br>`N =` normal<br>`R =` rotates 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from bottom up, 270 degrees<br>**Default:** `N` or the last`^FW` value|
|`h =` character height (in<br>dots)|**Default:**<br>Specifies magnification by`w` (character width) or the last accepted`^CF`<br>value. Uses the base height if none is specified.<br>•<br>Scalable - The value is the height in dots of the entire character block.<br>Magnification factors are unnecessary, because characters are scaled.<br>•<br>Bitmapped - The value is rounded to the nearest integer multiple of<br>the font’s base height, then divided by the font’s base height to give a<br>magnification nearest limit.|
|`w =` width (in dots)|**Default:** Specifies magnification by`h` (height) or the last accepted`^CF`<br>value. Specifies the base width is used if none is specified.<br>•<br>Scalable -  The value is the width in dots of the entire character block.<br>Magnification factors are unnecessary, because characters are scaled.<br>•<br>Bitmapped - The value rounds to the nearest integer multiple of the<br>font’s base width, then divided by the font’s base width to give a<br>magnification nearest limit.|
|`d =` drive location of font|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R`:|
|`f =` font name|**Values:** any valid font<br>**Default:** if an invalid or no name is entered, the default set by`^CF` is used If<br>no font has been specified in`^CF`, font A is used.<br>The font named carries over on all subsequent`^A@` commands without a<br>font name.|
|`x =` extension<br>`.TTE` is only supported in<br>firmware version V60.14.x,<br>V50.14.x, or later.|**Values:**<br>`.FNT` = font<br>`.TTF =` TrueType Font<br>`.TTE =` TrueType Extension|


62


ZPL Commands


**Example:** This example identifies the purpose of each line of code for this label:


1 Starts the label format.

2 Searches non-volatile printer memory ( `B:` ) for `CYRI_UB.FNT` . When the font is found, the `^A@`
command sets the print orientation to normal and the character size to 50 dots by 50 dots.

3 Sets the field origin at 100,100.

4 Prints the field data, **Zebra Printer Fonts** on the label.

5 Calls the font again and character size is decreased to 40 dots by 40 dots.

6 Sets the new field origin at 100,150.

7 Prints the field data, **This uses the B:CYRI_UB.FNT** on the label.

8 Ends the label format.


For reference, see Zebra Code Page 850 — Latin Character Set on page 1548, Fonts and Barcodes on
page 1562, and ASCII on page 1560.


63
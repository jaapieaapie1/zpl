# ^A




ZPL Commands


The `^A` command specifies the font to use in a text field. `^A` designates the font for the current `^FD`
statement or field. The font specified by `^A` is used only once for that `^FD` entry. If a value for `^A` is not
specified again, the default `^CF` font is used for the next `^FD` entry.


**Scalable/Bitmapped Font**


**Format:** ^Afo,h,w







|Parameter|Details|
|---|---|
|`f =` font name|**Values:** `A` through`Z`, and`0` to`9`<br>Any font in the printer (downloaded, EPROM, stored fonts, fonts`A` through`Z` and`0`<br>to`9`).<br>**IMPORTANT:** Parameter`f` is required. If`f` is omitted it defaults to the last<br>value of the`^CF` command.|
|`o =` field<br>orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from bottom up, 270 degrees<br>**Default:** the last accepted`^FW` value or the`^FW` default|
|`h =` Character<br>Height (in dots)|**Scalable**<br>**Values:** `10` to`32000`<br>**Default:** last accepted`^CF`<br>**Bitmapped**<br>**Values:** multiples of height from`1` to`10` times the standard height, in increments of<br>1<br>**Default:** last accepted`^CF`|
|`w =` width (in dots)|**Scalable**<br>**Values:** `10` to`32000`<br>**Default:** last accepted`^CF`<br>**Bitmapped**<br>**Values:** multiples of width from`1` to`10` times the standard width, in increments of 1<br>**Default:** last accepted`^CF`|


**Scalable Font Command**


**Example:** This is an example of a scalable font command:


60




ZPL Commands


**Bitmap Font Command**


**Example:** This is an example of a bitmap font command:


**Comments:**


Fonts are built using a matrix that defines standard height-to-width ratios. If you specify only the height or
width value, the standard matrix for that font automatically determines the other value. If the value is not
given or a 0 (zero) is entered, the height or width is determined by the standard font matrix.


This command interacts with the justification parameters of `^FO` and `^FT` and with the field direction
parameter of `^FP` . For output and examples, see Field Interactions on page 1588.


61
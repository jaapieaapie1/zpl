# ^PA




ZPL Commands


The `^PA` command is used to configure advanced text layout features.


**Advanced Text Properties**


This command is available only for printers with firmware version V60.14.x, V50.14.x, or later.

**Format:** `^PAa,b,c,d`





|Parameters|Details|
|---|---|
|`a =` default glyph|This determines whether the default glyph is a space character or the default<br>glyph of the base font, which is typically a hollow box.<br>**Values:**<br>`0 =` off (space as default glyph)<br>`1 =` on (default glyph of font is used, often a hollow box, but depends on the<br>font.)<br>**Default:** `0`|
|`b =` bidirectional text<br>layout|This determines whether the bidirectional text layout is turned on or off.<br>**Values:**<br>`0 =` off<br>`1 =` on<br>**Default:** `0`|
|`c =` character<br>shaping|This determines whether character shaping is turned on or off.<br>**Values:**<br>`0 =` off<br>`1 =` on<br>**Default:** `0`|
|`d =` OpenType table<br>support|This determines whether the OpenType support is turned on or off.<br>**Values:**<br>`0 =` off<br>`1 =` on<br>**Default:** `0`|


314
# ^TB




ZPL Commands


The `^TB` command prints a text block with defined width and height. The text block has an automatic wordwrap function. If the text exceeds the block height, the text is truncated. This command supports complex
text layout features.


**Text Blocks**


This command is available only for printers with firmware version V60.14.x, V50.14.x, or later.


**NOTE:** `^TB` is the preferred command for printing fields or blocks of text, instead of `^FB` .


**Format:** `^TBa,b,c`

|Parameters|Details|
|---|---|
|`a =` block rotation|**Values:**<br>`N =` normal<br>`R =` rotate 90 degrees clockwise<br>`I =` invert 180 degrees<br>`B =` read from bottom up-270 degrees<br>**Default:** whatever was specified by the last`^A` (which has the default of<br>`^FW`)|
|`b =` block width in dots|**Values:**<br>`1` to the width of the label in dots<br>**Default:** 1 dot|
|`c =` block height in dots|**Values:**<br>`1` to the length of the label in dots<br>**Default:** 1 dot|



**Comments:** Facts about the `^TB` command:

- Justification of `^TB` command comes from `^FO, ^FT`, or `^FN` command. If no justification is determined
then the default is auto justification.


- Data between < and > is processed as an escape sequence; for example, **<< >** will print **<** .

- The `^TB` command has an automatic word-wrap function. Soft hyphens do not print and are not used as
a line break position.


355
# ^BX




ZPL Commands


The `^BX` command creates a two-dimensional matrix symbology made up of square modules arranged
within a perimeter finder pattern.


**Data Matrix Barcode**

**Format:** `^BXo,h,s,c,r,f,g,a`







|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` dimensional height<br>of individual symbol<br>elements|**Values:** 1 to the width of the label<br>The individual elements are square — this parameter specifies both module<br>and row height. If this parameter is zero (or not given), the h parameter (bar<br>height) in`^BY` is used as the approximate symbol height.|
|`s =` quality level|**Values: 0, 50, 80, 100, 140, 200**<br>**Default:** `0`<br>**Quality** refers to the amount of data that is added to the symbol for error<br>correction. The AIM specification refers to it as the ECC value. ECC 50, ECC<br>80, ECC 100, and ECC 140 use convolution encoding; ECC 200 uses Reed-<br>Solomon encoding. For new applications, ECC 200 is recommended. ECC<br>000-140 should be used only in closed applications where a single party<br>controls both the production and reading of the symbols and is responsible<br>for overall system performance.|
|`c =` columns to encode|**Values:** `9` to`49`<br>Odd values only for quality 0 to 140 (10 to 144); even values only for quality<br>200.<br>Odd values only for quality 0 to 140 (10 to 144); even values only for quality<br>200. The number of rows and columns in the symbol is automatically<br>determined. You might want to force the number of rows and columns to a<br>larger value to achieve uniform symbol size. In the current implementation,<br>quality 0 to 140 symbols are square, so the larger of the rows or columns<br>supplied is used to force a symbol to that size. If you attempt to force the<br>data into too small of a symbol, no symbol is printed. If a value greater<br>than 49 is entered, the rows or columns value is set to zero, and the size<br>is determined normally. If an even value is entered, it generates INVALID-<br>P (invalid parameter). If a value is less than 9 but not 0, or if the data is too<br>large for the forced size, no symbol prints; if`^CV` is active, INVALID-L prints.|
|`r =` rows to encode|**Values:** `9` to`49`|


144


ZPL Commands






|Parameters|Details|
|---|---|
|`f =` format ID (0 to 6) —<br>not used with quality set<br>at 200|**Values:**<br>`1 =` field data is numeric + space (0..9,”) – No \&’’<br>`2 =` field data is uppercase alphanumeric + space (A..Z,’’) – No \&’’<br>`3 =` field data is uppercase alphanumeric + space, period, comma, dash,<br>and slash (0..9,A..Z,“.-/”)<br>`4 =` field data is upper-case alphanumeric + space (0..9,A..Z,’’) – no \&’’<br>`5 =` field data is full 128 ASCII 7-bit set<br>`6 =` field data is full 256 ISO 8-bit set<br>**Default:** `6`|
|`g =` escape sequence<br>control character|**Values:** any character<br>**Default:** `~` (tilde)<br>This parameter is used only if quality 200 is specified. It is the escape<br>character for embedding special control sequences within the field data.<br>A value must always be specified when using the escape sequence control<br>character. If no value is entered, the command is ignored.<br>The`g` parameter will continue to be underscore (`_`) for anyone with<br>firmware version: V60.13.0.12, V60.13.0.12Z, V60.13.0.12B, V60.13.0.12ZB, or<br>later.|
|`a =` aspect ratio<br>The`a` parameter is only<br>supported in V60.16.5Z<br>and V53.16.5Z or later.|**Values:**<br>`1 =` square<br>`2 =` rectangular<br>**Default:** `1`|



**Example:** This is an example of a square Data Matrix barcode:


145


ZPL Commands


**Example:** This is an example of a rectangle Data Matrix bar code:


**Effects of ^BY on ^BX**

`w =` **module width** (no effect)

`r =` **ratio** (no effect)

`h =` **height of symbol**

If the dimensions of individual symbol elements are not specified in the `^BY` command, the height of the
symbol value is divided by the required rows/columns, rounded, limited to a minimum value of one, and
used as the dimensions of individual symbol elements.


**Field Data (^FD) for ^BX**


**Quality 000 to 140**


- The **\&** and **||** can be used to insert carriage returns, line feeds, and backslash, similar to the PDF417.
Other characters in the control character range can be inserted only by using `^FH` . Field data is limited
to 596 characters for quality **0** to **140** . Excess field data causes no symbol to print; if `^CV` is active,
INVALID-L prints. The field data must correspond to a user-specified format ID or no symbol prints; if
`^CV` is active, INVALID-C prints.

- The maximum field sizes for quality **0** to **140** symbols are shown in the table in the `g` parameter.

**Quality 200**


- If more than 3072 bytes are supplied as field data, it is truncated to 3072 bytes. This limits the maximum
size of a numeric Data Matrix symbol to less than the 3116 numeric characters that the specification
would allow. The maximum alphanumeric capacity is 2335, and the maximum 8-bit byte capacity is
1556.

- If `^FH` is used, field hexadecimal processing takes place before the escape sequence processing
described below.


- The underscore is the default escape sequence control character for quality 200 field data. A different
escape sequence control character can be selected by using parameter g in the `^BX` command.


146


ZPL Commands


The information that follows applies to firmware versions: V60.13.0.12, V60.13.0.12Z, V60.13.0.12B,
V60.13.0.12ZB, or later. The input string escape sequences can be embedded in quality 200 field data
using the ASCII 95 underscore character ( _ ) or the character entered in parameter `g` :

- `_X` is the shift character for control characters (e.g., `_@=NUL,_G=BEL,_0 is PAD` )

- `_1` to `_3` for FNC characters 1 to 3 (explicit FNC4, upper shift, is not allowed)

- FNC2 (Structured Append) must be followed by nine digits, composed of three-digit numbers with
values between 1 and 254, that represent the symbol sequence and file identifier (for example, symbol
3 of 7 with file ID 1001 is represented by `_2214001001` )

- 5NNN is code page NNN where NNN is a three-digit code page value (for example, Code Page 9 is
represented by `_5009` )

- `_dNNN` creates ASCII decimal value NNN for a code word (must be three digits)

- _ in data is encoded by __ (two underscores)


The information that follows applies to all other versions of firmware. The input string escape sequences
can be embedded in quality 200 field data using the ASCII 7E tilde character (~) or the character entered in
the parameter `g` :

- `~X` is the shift character for control characters (e.g., `~@=NUL,~G=BEL,~0 is PAD` )

- `~1` to `~3` for FNC characters 1 to 3 (explicit FNC4, upper shift, is not allowed)

- FNC2 (Structured Append) must be followed by nine digits, composed of three-digit numbers with
values between 1 and 254, that represent the symbol sequence and file identifier (for example, symbol
3 of 7 with file ID 1001 is represented by `~2214001001` )

- 5NNN is code page NNN where NNN is a three-digit code page value (for example, Code Page 9 is
represented by `~5009` )

- `~dNNN` creates ASCII decimal value NNN for a code word (must be three digits)

- ~ in data is encoded by a ~ (tilde)


147
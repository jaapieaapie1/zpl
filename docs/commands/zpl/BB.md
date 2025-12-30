# ^BB




ZPL Commands


The `^BB` command produces a two-dimensional, multirow, stacked symbology. It is ideally suited for
applications that require large amounts of information.


**CODABLOCK Barcode**


Depending on the mode selected, the code consists of one to 44 stacked rows. Each row begins and ends
with a start and stop pattern.


- CODABLOCK A supports variable print ratios.


- CODABLOCK E and F support only fixed print ratios.


**IMPORTANT:** [If additional information about this bar code is required, go towww.aimglobal.org.](http://www.aimglobal.org)


**Format:** `^BBo,h,s,c,r,m`











|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** `N`|
|`h =` bar code height for<br>individual rows (in dots)|**Values:** `2` to`32000`<br>**Default:** `8` This number, multiplied by the module, equals the height of the<br>individual row in dots.|
|`s =` security level|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`<br>Security level determines whether symbol check-sums are generated<br>and added to the symbol. Checksums are never generated for single-row<br>symbols. This can be turned off only if the parameter`m` is set to`A`.|
|`c =` number of characters<br>per row (data columns)|**Values:** `2` to`62` characters<br>This is used to encode a CODABLOCK symbol. It gives you control over the<br>width of the symbol.|


90




ZPL Commands







|Parameters|Details|
|---|---|
|`r =` number of rows to<br>encode-|**Values:**<br>for CODABLOCK A:`1` to`22`<br>for CODABLOCK E and F:`2` to`4`<br>•<br>If values for c and r are not specified, a single row is produced.<br>•<br>If a value for r is not specified, and c exceeds the maximum range, a<br>single row equal to the field data length is produced.<br>•<br>If a value for c is not specified, the number of characters per row is<br>derived by dividing the field data by the value of`r`.<br>•<br>If the s parameter is set to the default of Y, then the checksum<br>characters that are included count as two data characters. For example,<br>if c = 6, r is set to 3 and s is set to N, then up to 18 characters can be<br>used (6 x 3). However, if s is set to Y, then only 16 characters can be<br>used.<br>•<br>If the data field contains primarily numeric data, fewer than the specified<br>rows might be printed. If the field data contains several shift and code-<br>switch characters, more than the specified number of rows might be<br>printed.|
|`m =` mode|**Values:** `A`, `E`, `F`<br>CODABLOCK A uses the Code 39 character set.<br>CODABLOCK F uses the Code 128 character set.<br>CODABLOCK E uses the Code 128 character set and automatically adds<br>FNC1.<br>**Default:** `F`|


**Example**


This is an example of a CODABLOCK barcode:


91


ZPL Commands


**Special Considerations for the** **`^BY`** **Command When Using** **`^BB`**

The parameters for the `^BYw,r,h` command, when used with a `^BB` code, are as follows:

`w =` **module width (in dots)**

**Values:** `2` to `10` (CODABLOCK A only)

**Default:** `2`

`r =` **ratio**

**Fixed Value:** `3` (ratio has no effect on CODABLOCK E or F)

`h =` **height of bars (in dots)**

**Values:** `1` to `32,32000`

**Default:** `10`

CODABLOCK uses this as the overall symbol height only when the row height is not specified in the `^BB h`
parameter.


**Special Considerations for** **`^FD`** **Character Set When Using** **`^BB`**

The character set sent to the printer depends on the mode selected in parameter m.


**CODABLOCK A:** CODABLOCK A uses the same character set as Code 39. If any other character is used in
the `^FD` statement, either no barcode is printed or an error message is printed (if `^CV` is active).

**CODABLOCK E:**


The Automatic Mode includes the full ASCII set except for those characters with special meaning to
the printer. Function codes or the Code 128 Subset A < `nul` - character can be inserted using the `^FH`
command.

```
 <fnc1> = 80 hex <fnc3> = 82 hex

```

92


ZPL Commands

|<fnc2> = 81 hex|<fnc4> = 83 hex|
|---|---|
|`<nul> = 84 hex`||



For any other character above 84 hex, either no barcode is printed or an error message is printed (if `^CV` is
active).


**CODABLOCK F:** CODABLOCK F uses the full ASCII set, except for those characters with special meaning

|CODABLOCK F: CODABLOCK F uses the full ASCII se to the printer. Function codes or the Code 128 Subset command.|et, except for those characters with special meaning A <nul> character can be inserted using the ^FH|
|---|---|
|`<fnc1> = 80 hex`|`<fnc3> = 82 hex`|
|`<fnc2> = 81 hex`|`<fnc4> = 83 hex`|
|`<nul> = 84 hex`||



93
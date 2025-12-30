# ^BI




ZPL Commands


The `^BI` command is a discrete, self-checking, continuous numeric symbology. The Industrial 2 of 5
barcode has been in use the longest of the 2 of 5 family of barcodes. Of that family, the Standard 2 of 5
( `^BJ` ) and Interleaved 2 of 5 ( `^B2` ) barcodes are also available in ZPL II.


**Industrial 2 of 5 Bar Codes**


With Industrial 2 of 5, all of the information is contained in the bars. Two bar widths are employed in this
code, the wide bar measuring three times the width of the narrow bar.

- `^BI` supports a print ratio of 2.0:1 to 3.0:1.

- Field data ( `^FD` ) is limited to the width (or length, if rotated) of the label.


**IMPORTANT:** If additional information about this barcode is required, refer to www.aimglobal.org.


**Format:** `^BIo,h,f,g`



|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` bar code height (in<br>dots)|**Values:**<br>`1` to`32000`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|


**Example**





This is an example of an Industrial 2 of 5 barcode:


114


ZPL Commands


115
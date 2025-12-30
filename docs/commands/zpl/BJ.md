# ^BJ




ZPL Commands


The `^BJ` command is a discrete, self-checking, continuous numeric symbology.


**Standard 2 of 5 Bar Code**


With Standard 2 of 5, all of the information is contained in the bars. Two bar widths are employed in this
code, the wide bar measuring three times the width of the narrow bar.

- `^BJ` supports a print ratio of 2.0:1 to 3.0:1.

- Field data ( `^FD` ) is limited to the width (or length, if rotated) of the label.


**IMPORTANT:** If additional information about this bar code is required, refer to
www.aimglobal.org.

**Format:** `^BJo,h,f,g`



|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` Barcode height (in<br>dots)|**Values:** `1` to`32000`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|


**Example**





This is an example of a Standard 2 of 5 barcode:


116


ZPL Commands


117
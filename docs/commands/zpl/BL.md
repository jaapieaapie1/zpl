# ^BL




ZPL Commands


The `^BL` command is a special application of Code 39 used by the Department of Defense. LOGMARS is
an acronym for Logistics Applications of Automated Marking and Reading Symbols.


**LOGMARS Barcode**

- `^BL` supports a print ratio of 2.0:1 to 3.0:1.

- Field data ( `^FD` ) is limited to the width (or length, if rotated) of the label. Lowercase letters in the `^FD`
string are converted to the supported uppercase LOGMARS characters.


**IMPORTANT:** [If additional information about this barcode is required, refer to www.aimglobal.org.](http://www.aimglobal.org)


**Format:** `^BLo,h,g`



|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` Barcode height (in<br>dots)|**Values:** `1` to`32000`<br>**Default:** value set by`^BY`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|


**Example**





This is an example of a LOGMARS barcode:



120




ZPL Commands


**Comments:** The LOGMARS barcode produces a mandatory check digit using Mod 43 calculations. For
further information on the Mod 43 check digit, go toMod 10 Check Digit on page 1573.


121
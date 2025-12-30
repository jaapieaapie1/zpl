# ^BP




ZPL Commands


The `^BP` command is a pulse-width modulated, continuous, non-self-checking symbology.


**Plessey Barcode**


Each character in the Plessey barcode is composed of eight elements: four bars and four adjacent spaces.

- `^BP` supports a print ratio of 2.0:1 to 3.0:1.

- Field data ( `^FD` ) is limited to the width (or length, if rotated) of the label.


**IMPORTANT:** If additional information about this bar code is required, refer to
[www.aimglobal.org.](http://www.almglobal.org)

**Format:** `^BPo,e,h,f,g`



|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`e =` print check digit|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|
|`h =` barcode height (in<br>dots)|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|
|`f =` print interpretation<br>line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|


**Example**





This is an example of a Plessey barcode:



126


ZPL Commands


127
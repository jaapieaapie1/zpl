# ^BZ




ZPL Commands


The POSTAL barcode is used to automate the handling of mail. POSTAL codes use a series of tall and
short bars to represent the digits.

- `^BZ` supports a print ratio of 2.0:1 to 3.0:1.

- Field data ( `^FD` ) is limited to the width (or length, if rotated) of the label and by the barcode
specification.


**IMPORTANT:** If additional information about the POSTAL and PLANET barcode is required, go to
[www.aimglobal.org, or contact the United States Postal Service at](http://www.aimglobal.org) [http://pe.usps.gov. If additional](http://pe.usps.gov)
information about the INTELLIGENT MAIL barcode is required, see: [http://ribbs.usps.gov/](http://ribbs.usps.gov/OneCodeSolution)
[OneCodeSolution.](http://ribbs.usps.gov/OneCodeSolution)

**Format:** `^BZo,h,f,g,t`







|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` Barcode height (in<br>dots)|**Values:** `1` to`32000`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|
|`t =` Postal code type|**Values:**<br>`0 =` Postnet barcode<br>`1 =` Plant barcode<br>`2 =` Reserved<br>`3 =` USPS Intelligent Mail barcode<br>**Default:** `0`|


**Examples**


This is an example of a POSTNET barcode:


150




ZPL Commands


This is an example of a USPS Intelligent Mail barcode:


151
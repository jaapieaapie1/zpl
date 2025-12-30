# ^BM




ZPL Commands


The `^BM` command is a pulse-width modulated, continuous, non-self-checking symbology. It is a variant of
the Plessey barcode ( `^BP` ).


**MSI Bar Code**


Each character in the MSI barcode is composed of eight elements: four bars and four adjacent spaces.

- `^BM` supports a print ratio of 2.0:1 to 3.0:1.

- For the barcode to be valid, field data ( `^FD` ) is limited to 1 to 14 digits when parameter `e` is `B`, `C`, or `D` .
`^FD` is limited to 1 to 13 digits when the parameter `e` is `A`, plus a quiet zone.


**IMPORTANT:** If additional information about this bar code is required, refer to
[www.aimglobal.org.](http://www.aimglobal.org)

**Format:** `^BMo,e,h,f,g,e2`





|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`e =` check digit selection|**Values:**<br>`A =` no check digits<br>`B =` 1 Mod 10<br>`C =` 2 Mod 10<br>`D =` 1 Mod 11 and 1 Mod 10<br>**Default:** `B`|
|`h =` Barcode height (in<br>dots)|**Values:**<br>`1` to`32000`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|


122


ZPL Commands



|Parameters|Details|
|---|---|
|`e2 =` inserts check digit<br>into the interpretation line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|


**Example**





This is an example of an MSI barcode:



123
# ^BK




ZPL Commands


The ANSI Codabar barcode is used in a variety of information processing applications such as libraries, the
medical industry, and overnight package delivery companies. This barcode is also known as USD-4 code,
NW-7, and 2 of 7 code. It was originally developed for retail price labeling.


**ANSI Codabar Bar Code**


Each character in this code is composed of seven elements: four bars and three spaces. Codabar bar
codes use two character sets, numeric and control (start and stop) characters.

- `^BK` supports a print ratio of 2.0:1 to 3.0:1.

- Field data ( `^FD` ) is limited to the width (or length, if rotated) of the label.


**IMPORTANT:** If additional information about this barcode is required, refer to www.aimglobal.org.


**Format:** `^BKo,e,h,f,g,k,l`





|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`e =` check digit|**Fixed Value:** `N`|
|`h =` Barcode height (in<br>dots)|**Values:** `1` to`32000`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`g =` print interpretation<br>line above code|**Values:**<br>`Y =` no<br>`Y =` yes<br>**Default:** `N`|
|`k =` designates a start<br>character|**Values:**<br>`A,B`,<br>`C`,<br>`D`<br>**Default:** `A`|


118


ZPL Commands



|Parameters|Details|
|---|---|
|`l =` designates stop<br>character|**Values:**<br>`A,B`,<br>`C`,<br>`D`<br>**Default:** `A`|


**Example**





This is an example of an ANSI Codabar barcode:


119
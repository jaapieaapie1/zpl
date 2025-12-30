# ^BS




ZPL Commands


The `^BS` command is the two-digit and five-digit add-on used primarily by publishers to create barcodes for
ISBNs (International Standard Book Numbers). These extensions are handled as separate bar codes.


**UPC/EAN Extensions**

The `^BS` command is designed to be used with the UPC-A barcode ( `^BU` ) and the UPC-E barcode ( `^B9` ).

- `^BS` supports a fixed print ratio.

- Field data ( `^FD` ) is limited to exactly two or five characters. ZPL II automatically truncates or pads on the
left with zeros to achieve the required number of characters.


**IMPORTANT:** [If additional information about this barcode is required, go to www.aimglobal.org.](http://www.aimglobal.org)


**Format:** `^BSo,h,f,g`



|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` Barcode height (in<br>dots)|**Values:** `1` to`32000`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|


**Example**





This is an example of a UPC/EAN Two-digit barcode:


137


ZPL Commands


**Example**


This is an example of a UPC/EAN Five-digit barcode:


Care should be taken in positioning the UPC/EAN extension with respect to the UPC-A or UPC-E code to
ensure the resulting composite code is within the UPC specification.


**Example**

For UPC codes, with a module width of `2` (default), the field origin offsets for the extension are:

This is an example of a UPC-A:


**Example**


This is an example of a UPC-E:


138


ZPL Commands


Additionally, the barcode height for the extension should be 27 dots (0.135 inches) shorter than that of the
primary code. A primary UPC code height of 183 dots (0.900 inches) requires an extension height of 155
dots (0.765 inches).


**Example**


This example illustrates how to create a normal UPC-A barcode for the value 7000002198 with an
extension equal to 04414:


139
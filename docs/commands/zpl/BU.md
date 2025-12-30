# ^BU




ZPL Commands


The `^BU` command produces a fixed length, numeric symbology. It is primarily used in the retail industry for
labeling packages. The UPC-A barcode has 11 data characters. The 6 dot/mm, 12 dot/mm, and 24 dot/mm
printheads produce the UPC-A barcode (UPC/EAN symbologies) at 100 percent size. However, an 8 dot/
mm printhead produces the UPC/EAN symbologies at a magnification factor of 77 percent.


**UPC-A Barcode**

- `^BU` supports a fixed print ratio.

- Field data ( `^FD` ) is limited to exactly 11 characters. ZPL II automatically truncates or pads on the left with
zeros to achieve the required number of characters.


**IMPORTANT:** [If additional information about this barcode is required, go to www.aimglobal.org.](http://www.almglobal.org)


**Format:** `^BUo,h,f,g,e`







|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` Barcode height (in<br>dots)|**Values:** `1` to`9999`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|
|`e =` print check digit|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|


The font style of the interpretation line depends on the modulus (width of narrow bar) selected in `^BY` . Zero
is not allowed.


- **6 dot/mm printer:** a modulus of 2 dots or greater prints with an OCR-B interpretation line; a modulus of
1 dot prints font A.


- **8 dot/mm printer:** a modulus of 3 dots or greater prints with an OCR-B interpretation line; a modulus of
1 or 2 dots prints font A.


142


ZPL Commands


- **12 dot/mm printer:** a modulus of 5 dots or greater prints with an OCR-B interpretation line; a modulus of
1, 2, 3, or 4 dots prints font A.


- **24 dot/mm printer:** a modulus of 9 dots or greater prints with an OCR-B interpretation line; a modulus
of 1 to 8 dots prints font A.


**Example**


This is an example of a UPC-A barcode with extension:


**Comments:** The UPC-A bar code uses the Mod 10 check digit scheme for error checking. For further
information on Mod 10, see Mod 10 Check Digit on page 1573.


143
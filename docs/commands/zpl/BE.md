# ^BE




ZPL Commands


The `^BE` command is similar to the UPC-A barcode. It is widely used throughout Europe and Japan in the
retail marketplace.


**EAN-13 Barcode**


The EAN-13 barcode has 12 data characters, one more data character than the UPC-A code. An EAN-13
symbol contains the same number of bars as the UPC-A but encodes a 13th digit into a parity pattern of the
left-hand six digits. This 13th digit, in combination with the 12th digit, represents a country code.

- `^BE` supports fixed print ratios.

- Field data ( `^FD` ) is limited to exactly 12 characters. ZPL II automatically truncates or pads on the left with
zeros to achieve the required number of characters.


- When using JAN-13 (Japanese Article Numbering), a specialized application of EAN-13, the first two nonzero digits sent to the printer must be 49.


**NOTE:** Use Interleaved 2 of 5 for UCC and EAN 14.


**IMPORTANT:** [If additional information about this barcode is required, refer to www.aimglobal.org.](http://www.aimglobal.org)


**Format:** `^BEo,h,f,g`



|Parameters|Details|
|---|---|
|`o =` orientation|**Value:**<br>`N =` normal<br>`R =` rotated 90 degrees (clockwise)<br>`I =` inverted 180 degrees<br>`B =` read from the bottom up, 270 degrees<br>**Default:** current`^FW` value|
|`h =` barcode height (in<br>dots)|**Value:**<br>`1` to`32000`<br>**Default:** value set by`^BY`|
|`f =` print interpretation<br>line|**Value:**<br>`N =` no<br>`Y =` yes<br>**Default:** `Y`|
|`g =` print interpretation<br>line above code|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|


**Example**





This is an example of an EAN-13 barcode:



109


ZPL Commands


**Comments:** The EAN-13 barcode uses the Mod 10 check-digit scheme for error checking. For more
information on Mod 10, see Mod 10 Check Digit on page 1573.


110
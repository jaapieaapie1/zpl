# ^FP




ZPL Commands


The `^FP` command allows vertical and reverse formatting of the font field, commonly used for printing
Asian fonts.


**Field Parameter**

**Format:** `^FPd,g`






|Parameters|Details|
|---|---|
|`d =` direction|**Values:**<br>`H =` horizontal printing (left to right)<br>`V =` vertical printing (top to bottom)<br>`R =` reverse printing (right to left)<br>**Default:** `H`|
|`g =` additional inter-<br>character gap (in dots)|**Values:** `0` to`9999`<br>**Default:** `0` if no value is entered|



**Example:** This is an example of how to implement reverse and vertical print:


For vertical and reverse printing directions, combining semantic clusters are used to place characters.

This command interacts with the justification parameters of `^FO` and `^FT` and with the rotation parameter
of `^A` . For output and examples, see Field Interactions on page 1588.


202
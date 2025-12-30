# ^MF




ZPL Commands


The `^MF` command dictates what happens to the media at power-up and at head-close after the error
clears.


**Media Feed**

**Format:** `^MFp,h`






|Parameters|Details|
|---|---|
|`p =` feed action at power-<br>up|**Values:**<br>`F =` feed to the first web after sensor<br>`C =` (see~JC definition)<br>`L =` (see~JL definition)<br>`N =` no media feed<br>`S =` short calibration1<br>**Default:** `C`|
|`h =` feed action after<br>closing printhead|**Values:**<br>`F =` feed to the first web after sensor<br>`C =` (see~JC definition)<br>`L =` (see~JL definition)<br>`N =` no media feed<br>`S =` short calibration1<br>**Default:** `C`|



1. These values are supported only on Xi4, RXi4, XiIIIPlus, PAX, ZM400/ZM600, RZ400/RZ600, and S4M
printers.

**Comments:** It is important to remember that if you choose the `N` setting, the printer assumes that the
media and its position relative to the printhead are the same as before power was turned off or the
printhead was opened. Use the `^JU` command to save changes.


302
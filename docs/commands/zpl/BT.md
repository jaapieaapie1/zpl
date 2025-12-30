# ^BT




ZPL Commands


The `^BT` bar code is the standard for the TCIF can tag telecommunications equipment.


**TLC39 Barcode**


The TCIF CLEI code, which is the MicroPDF417 barcode, is always four columns. The firmware must
determine what mode to use based on the number of characters to be encoded.

**Format:** `^BTo,w1,r1,h1,w2,h2`



|Parameters|Details|
|---|---|
|`o =` orientation|**Values:**<br>`N =` normal<br>`R =` rotated<br>`I =` inverted<br>`B =` bottom up|
|`w1 =` width of the Code<br>39 bar code|**Values: (in dots):** `1` to`10`<br>**Default: (600 dpi printers):** `4`<br>**Default: (200- and 300 dpi printer):** `2`|
|`r1 =` wide to narrow bar<br>width ratio the Code 39<br>bar code|**Values:** `2.0 to 3.0(increments of 0.1)`<br>**Default:** `2.0`|
|`h1 =` height of the Code<br>39 bar code|**Values: (in dots):** `1` to`9999`<br>**Default: (600 dpi printer):** `120`<br>**Default: (300 dpi printer):** `60` **Default: (200 dpi printer):** `40`|
|`h2 =` row height of the<br>MicroPDF417 bar code|**Values: (in dots):** `1` to`255`<br>**Default: (600 dpi printer):** `8`<br>**Default: (200- and 300 dpi printers):** `4`|
|`w2 =` narrow bar width of<br>the MicroPDF417 bar code|**Values: (in dots):** `1` to`10`<br>**Default: (600 dpi printer):** `4`<br>**Default: (200- and 300 dpi printers):** `2`|


**How to Print TLC39 Barcode**





**Example:** This is an example on how to print TLC39 barcode. The callouts identify the key components and
are followed by a detailed description below:


Use the command defaults to get results that are in compliance with TCIF industry standards; regardless of
printhead density.


140




ZPL Commands

|1|ECI Number. If the seventh character is not a comma, only Code 39 prints. This means if more<br>than 6 digits are present, Code 39 prints for the first six digits (and no Micro-PDF symbol is<br>printed).<br>• Must be 6 digits.<br>• Firmware generates invalid character error if the firmware sees anything but 6 digits.<br>• This number is not padded.|
|---|---|
|2|**Serial number.** The serial number can contain up to 25 characters and is variable length. The<br>serial number is stored in the Micro-PDF symbol. If a comma follows the serial number, then<br>additional data is used below.<br>•<br>If present, must be alphanumeric (letters and numbers, no punctuation).<br>This value is used if a comma follows the ECI number.|
|3|**Additional data.** If present, it is used for things such as a country code.<br>Data cannot exceed 150 bytes. This includes serial number commas.<br>•<br>Additional data is stored in the Micro-PDF symbol and appended after the serial number. A<br>comma must exist between each maximum of 25 characters in the additional fields.<br>•<br>Additional data fields can contain up to 25 alphanumeric characters per field.<br>The result is:|



141
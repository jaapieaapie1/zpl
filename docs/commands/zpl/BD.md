# ^BD




ZPL Commands


The `^BD` command creates a two-dimensional, optically read (not scanned) code. This symbology was
developed by UPS (United Parcel Service).


**UPS MaxiCode Bar Code**


Notice that there are no additional parameters for this code and it does not generate an interpretation
line. The `^BY` command has no effect on the UPS MaxiCode barcode. However, the `^CV` command can be
activated.


**Format:** ^BDm,n,t



|Parameters|Details|
|---|---|
|`m =` mode|**Values:**<br>`2 =` structured carrier message: numeric postal code (U.S.)<br>`3 =` structured carrier message: alphanumeric postal code (non-U.S.)<br>`4 =` standard symbol, secretary<br>`5 =` full EEC<br>`6 =` reader program, secretary<br>**Default:** `2`|
|`n =` symbol number|**Values:**<br>`1` to`8` can be added in a structured document<br>**Default:** `1`|
|`t =` total number of<br>symbols|**Values:**<br>`1` to`8`, representing the total number of symbols in this sequence<br>**Default:**`1`|


**Example**





This is an example of the UPS MAXICODE - MODE 2 barcode:


**Special Considerations for ^FD when Using ^BD**

The `^FD` statement is divided into two parts: a high-priority message ( `hpm` ) and a low-priority message
( `lpm` ). There are two types of high-priority messages. One is for a US Style Postal Code; the other is for a
non-U.S. Style Postal Code. The syntax for either of these high-priority messages must be exactly as shown
or an error message is generated.


106


ZPL Commands


**Format:** `^FD <hpm><lpm>`



|Parameters|Details|
|---|---|
|`<hpm> =` high-priority<br>message (applicable only<br>in Modes 2 and 3)|**Values:** `0` to`9`, except where noted<br>U.S. Style Postal Code (Mode 2)<br>`<hpm> =` aaabbbcccccdddd<br>`aaa =` three-digit class of service<br>`bbb =` three-digit country zip code<br>`ccccc =` five-digit zip code<br>`dddd =` four-digit zip code extension (if none exists, four zeros (0000) must<br>be entered)<br>non-U.S. Style Postal Code (Mode 3)<br>`<hpm> = aaabbbcccccc`<br>`aaa =` three-digit class of service<br>`bbb =` three-digit country zip code<br>`ccccc =` six-digit zip code (`A` through`Z` or`0` to`9`)|
|`<lpm> =` low priority<br>message (only applicable<br>in Modes 2 and 3)|GS is used to separate fields in a message (0x1D). RS is used to separate<br>format types (0x1E). EOT is the end of transmission characters.<br>**Message Header**  [)>RS<br>**Transportation Data**<br>**Format Header**  01GS96<br>**Tracking Number*** <tracking number><br>**SCAC***GS<SCAC><br>**UPS Shipper Number**  GS<shipper number><br>**Julian Day of Pickup**  GS<day of pickup><br>**Shipment ID Number**  GS<shipment ID number><br>**Package n/x**  GS<n/x><br>**Package Weight**  GS<weight><br>**Address Validation**  GS<validation><br>**Ship to Street Address**  GS<street address><br>**Ship to City**  GS<city><br>**Ship to State**  GS<state><br>**RS**  RS<br>**End of Message**  EOT<br>(***** Mandatory Data for UPS)|


**Comments:**






- The formatting of `<hpm>` and `<lpm>` apply only when using Modes 2 and 3. Mode 4, for example, takes
whatever data is defined in the `^FD` command and places it in the symbol.

- UPS requires that certain data be present in a defined manner. When formatting MaxiCode data for
UPS, always use uppercase characters. When filling in the **fields** in the `<lpm>` for UPS, follow the data
size and types specified in **Guide to Bar Coding with UPS** .


107


ZPL Commands


- If you do not choose a mode, the default is Mode 2. If you use non-U.S. Postal Codes, you probably get
an error message (invalid character or message too short). When using non-U.S. codes, use Mode 3.


- ZPL II doesn’t automatically change your mode based on the zip code format.

- When using special characters, such as GS, RS, or EOT, use the `^FH` command to tell ZPL II to use the
hexadecimal value following the underscore character ( _ ).


108
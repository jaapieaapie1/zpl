# ^SN




ZPL Commands


The `^SN` command allows the printer to index data fields by a selected increment or decrement value,
making the data fields increase or decrease by a specified value each time a label is printed. This can be
performed on 100 to 150 fields in a given format and can be performed on both alphanumeric and barcode
fields. A maximum of 12 of the right- most integers are subject to indexing.


**Serialization Data**


In x.13 and earlier, the first integer found when scanning from right to left starts the indexing portion of the
data field.


In x.14 and later, the first integer found when scanning from the end of the backing store towards the
beginning starts the indexing portion of the data field.


In x.13 and earlier, if the alphanumeric field to be indexed ends with an alpha character, the data is
scanned, character by character, from right to left until a numeric character is encountered. Serialization
takes place using the value of the first number found.


In x.14 and later, if the backing store of the alphanumeric field to be indexed ends with an alpha character,
the data is scanned, character by character, from the end of the backing store until a numeric character is
encountered. Serialization takes place using the value of the first number found.

**Format:** `^SNv,n,z`






|Parameters|Details|
|---|---|
|`v =` starting value|**Values:** 12-digits maximum for the portion to be indexed<br>**Default:** `1`|
|`n =` increment or<br>decrement value|**Values:** 12-digit maximum<br>**Default:** `1`<br>To indicate a decrement value, precede the value with a minus (–) sign.|
|`z =` add leading zeros (if<br>needed)|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|



**Example:** This example shows incrementing by a specified value:


340




ZPL Commands


**Comments:** Incrementing and decrementing takes place for each serial-numbered field when all replicates
for each serial number have been printed, as specified in the parameter `r` of the `^PQ` (print quality)
command.


If, during the course of printing serialized labels, the printer runs out of either paper or ribbon, the first
label printed (after the media or ribbon has been replaced and calibration completed) has the same serial
number as the **partial** label printed before the **out** condition occurred. This is done in case the last label
before the **out** condition did not fully print. This is controlled by the `^JZ` command.


**Using Leading Zeros**

In the `^SN` command, the `z` parameter determines if leading zeros are printed or suppressed. Depending
on which value is used ( `Y =` print leading zeros; `N =` do not print leading zeros), the printer either prints or
suppresses the leading zeros.

The default value for this parameter is `N` (do not print leading zeros).


**Print Leading Zeros**


In x.13 and earlier, the starting value consists of the right-most consecutive sequence of digits.


In x.14 and later, the starting value consists of the first number working backward in the backing store
consecutive sequence of digits.


The width (number of digits in the sequence) is determined by scanning from right to left until the first nondigit (space or alpha character) is encountered. To create a specific width, manually place leading zeros as
necessary.


**Suppressing Leading Zeros**


In x.13 and earlier, the starting value consists of the right-most consecutive sequence of digits, including
any leading spaces.


In x.14 or later, the starting value consists of the first number working backward in the backing store
consecutive sequence of digits, including any leading spaces.


341
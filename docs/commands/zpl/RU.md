# ^RU




ZPL RFID Commands


Use this command to read the TID (Tag ID) data from the current chip and format a unique serial
38-bit
number, which will be placed in the lower (least significant) 38 bits of the EPC code.


**Read Unique RFID Chip Serialization**

**Format:** `^RUa,b`

|Parameters|Details|
|---|---|
|`a` = prefix|Specifies the prefix in ASCII Binary<br>**Values:**Only ASCII characters 1 and 0 are accepted. Maximum of 38<br>characters.<br>The number of bits in the value specifies the length of the prefix. The prefix<br>is placed as the left-most (most significant) bits in the unique serial number.<br>If nothing is specified, the default value will be used.<br>**Default:**The MCS prefix is determined by the MDID in the TID of the chip<br>read:<br>100 = EM Micro<br>Impinj = 101<br>Alien = 110<br>NXP = 111|
|`b` = special character|Special character for serial number inclusion.<br>**Values:**Any ASCII character other than the current Command character,<br>Control character, Delimiter character, or any of the Real-Time Clock (RTC)<br>characters.<br>**Default:**`#`|



**NOTE:** Serial number inclusion:


One of several data elements can be included into any `^FD` data string in the same way that Real Time
Clock data is included. Use any of the commands below to include a data pattern based on the serial
number. These are defined using the default value for the Special Character.

`#S` = include 38-bit serial number derived from TID in decimal form.

`#H` = include 38-bit serial number derived from TID in hexadecimal form.

`#E` = include the entire 96-bit EPC code, including the 38-bit serial number derived from TID in decimal
form.

`#F` = include the entire 96-bit EPC code, including the 38-bit serial number derived from TID in
hexadecimal form.

`#P` = include the entire 96-bit EPC code, but use the tag’s preprogrammed, 38-bit SGTIN serial number in
decimal form.*

`#Q` = include the entire 96-bit EPC code, but use the tag’s preprogrammed, 38-bit SGTIN serial number in
hexadecimal form.*


437


ZPL RFID Commands


- If the EPC has been preprogrammed (typically by the manufacturer) with the chip-based RFID serialization
scheme, then the serialized data does not have to be written back to the EPC memory, which saves time.
`#P` and `#Q` simply format the data that is read from the EPC memory bank.

**Example:** Read the TID from the tag, create a serial number based on the tag type, write `12<serial`
`number (5 bytes)>000000000000` to the 96-bit EPC field, and print the serial number (in hex format)
on the label.

```
^XA
^RU
^FO10,10^A0N,50,50^FDSerial Number: #H^FS
^RFW,H^FD12#H^FS
^XZ

```

**Example:** Read the TID from the tag, create a serial number based on the tag type, write the serial number
to the EPC field (lower 38 bits) while maintaining the contents of the rest of the EPC memory, print `Serial`
`Number: <serial number in hex format>` on the label, and return `Serial Number: <serial`
`number in hex format>` to the host. Perform this operation on three label formats.

```
^XA
^RU
^FO10,10^A0N,50,50^FN1^FS
^FN1^FDSerial Number: #H^FS
^FH^HV1,24,,_0D_0A,L^FS
^RFW,H^FD#F^FS
^PQ3
^XZ

```

**Example:** Read the full EPC (already serialized) from the tag, print `Serial Number: <full EPC in`
`decimal format>` on the label, and return `Serial Number: <full EPC in decimal format>` to
the host.

```
^XA
^RU
^FO10,10^A0N,50,50^FN1^FS
^FN1^FDSerial Number: #P^FS
^FH^HV1,44,,_0D_0A,L^FS
^XZ

```

438
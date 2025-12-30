# ^RB




ZPL RFID Commands


Use this command to define the structure of EPC data, which can be read from or written to an RFID tag.
For more information about EPC specifications, refer to the EPC Global web site. All parameters in this
command are persistent and will be used in subsequent formats if not provided. The values are initially set
to the default values.


**Define EPC Data Structure**


RFID tags can have different partitions defined. This command specifies the number of partitions and how
many bits are in each partition.

**Format:** `^RBn,p0,p1,p2, ..., p15`






|Parameters|Details|
|---|---|
|`n` = total bit size of the<br>partitions|Specify the number of bits to include in the partitions.<br>**Values:** 1 to`n`, where`n` is the bit size of the tag.<br>**Default:** 96|
|`p0 ... p15` = partition<br>sizes|Specify the number of bits to include in the individual partitions. The<br>partition sizes must add up to the bit size specified for the previous<br>parameter. The largest individual partition size is 64 bits.<br>**Values:**1 to 64<br>**Default:**1|



**Example:** The following command specifies that there are 96 bits used with three fields. Fields 1, 2, and 3
contain 10, 26, and 60 bits, respectively.

```
^RB96,10,26,60

```

The ZPL code to encode a tag with this format would look like this:

```
^RFW,E^FD1000.67108000.1122921504606846976^FS

```

When the tag is being encoded, the tag stores the data in the following way:

- Field 1 contains `1000` . This value is stored in the first 10 bits

- Field 2 contains `67108000` . This value is stored in the next 26 bits.

- Field 3 contains `1122921504606846976` . This value is stored in the remaining 60 bits.

**Example:** The following command specifies that there are 64 bits used with eight fields.
8-bit

```
^RB64,8,8,8,8,8,8,8,8^FS

```

The ZPL code to encode a tag with this format would look like this:

```
^RFW,E^FD1.123.160.200.249.6.1.0^FS

```

When writing to the tag, each set of data is written in its respective 8-bit field.


**Example:** This example uses the SGTIN-96 standard, which defines 96-bit structure in the following way:


423


ZPL RFID Commands




















|Col1|Header|Filter Value|Partition|Company<br>Prefix Index|Item<br>Reference|Serial<br>Number|
|---|---|---|---|---|---|---|
|SGTIN-96|8 bits|3 bits|3 bits|20–40 bits|24 bits|38 bits|
|SGTIN-96|||||||




 - Capacity of Item Reference field varies with the length of the company prefix.


The ZPL code to encode a tag with this format would look like this:

```
^XA
^RB96,8,3,3,20,24,38^FS
^RFW,E^FD48,1,6,770289,10001025,1^FS
^XZ

```

These commands would put

- `48` in the header

- `1` as the filter value

- 6 as the partition (indicates a 20-bit prefix and 24-bit item reference)

- `770289` as the company prefix

- `10001025` as the item reference

- `1` as the serial number

To read this EPC data and print the results on the label, you would use the following code:

```
^XA
^RB96,8,3,3,20,24,38^FS
^FO50,50^A0N,40^FN0^FS
^FN0^RFR,E^FS
^XZ

```

The resulting label would look like this:


424
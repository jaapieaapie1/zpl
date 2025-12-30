# ~DE




ZPL Commands


The `~DE` command is used to download character encoding translation tables to a printer. This is essential
for printing non-standard or foreign language characters, particularly when using TrueType fonts.


**Download Encoding**


The standard encoding for TrueType Windows fonts is always Unicode. The ZPL II field data must be
converted from some other encoding to Unicode that the Zebra printer understands. The required
[translation tables are provided with font packs. Some tables can be downloaded from zebra.com.](http://www.zebra.com)

**Format:** `~DEd:o.x,s,data`

|Parameters|Details|
|---|---|
|`d =` location of table|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` name of table|**Values:** any valid name, up to 8 characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension|**Format:** `.DAT`|
|`s =` table size|**Values:** the number of memory bytes required to hold the Zebra<br>downloadable format of the font<br>**Default:** if an incorrect value or no value is entered, the command is<br>ignored|
|`data =` data string|**Values:** a string of ASCII hexadecimal values<br>**Default:** if no data is entered, the command is ignored|



**Example:** This is an example of how to download the required translation table:

```
~DER:JIS.DAT,27848,300021213001... (27848 two-digit hexadecimal values)

```

**Comments:** For more information on ZTools or ZebraNet Bridge, see the program documentation included
with the software.

For assistance with editing or adding mappings to `.DAT` tables, ZebraNet Bridge includes a `.DAT` table
editor in the font wizard.


Encoding scheme for the data sent to the printer is the second four character, and the encoding scheme
for the font is the first four characters throughout the `.DAT` file. The data must be ordered by the second
four characters (the encoding table).

**Example:** This is an example of a `.DAT` table. The table below the example identifies the elements:



~DEE:EXAMPLE.DAT,16,
00310041
00320042
00330043
00340044



1
2

3
4



Data must have 0041, 0042, 0043, and 0044 in order. Multiple pairs can be on the same line.


172


ZPL Commands


1 Input stream with `0041` will be mapped to 0031. The printer prints "1".

2 Input stream with `0042` will be mapped to 0032. The printer prints "2".

3 Input stream with `0043` will be mapped to 0033. The printer prints "3".

4 Input stream with `0044` will be mapped to 0034. The printer prints "4".


173
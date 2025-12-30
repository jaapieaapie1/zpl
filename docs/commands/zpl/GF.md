# ^GF




ZPL Commands


The `^GF` command allows you to download graphic field data directly into the printer’s bitmap storage
area. This command follows the conventions for any other field, meaning a field orientation is included. The
graphic field data can be placed at any location within the bitmap space.


**Graphic Field**

**Format:** `^GFa,b,c,d,data`







|Parameters|Details|
|---|---|
|`a =` compression type|**Values:**<br>`A =` ASCII hexadecimal (follows the format for other download commands)<br>`B =` binary (data sent after the`c` parameter is strictly binary)<br>`C =` compressed binary (data sent after the`c` parameter is in compressed<br>binary format. The data is compressed on the host side using Zebra’s<br>compression algorithm. The data is then decompressed and placed directly<br>into the bitmap.)<br>**Default:** `A`|
|`b =` binary byte count|**Values:** `1` to`99999`<br>This is the total number of bytes to be transmitted for the total image or<br>the total number of bytes that follow parameter`d`. For ASCII download, the<br>parameter should match parameter`c`. Out-of-range values are set to the<br>nearest limit.<br>**Default:** command is ignored if a value is not specified|
|`c =` graphic field<br>count|**Values:** 1 to 99999<br>This is the total number of bytes comprising the graphic format (width x<br>height), which is sent as parameter`d`. Count divided by bytes per row gives the<br>number of lines in the image. This number represents the size of the image,<br>not necessarily the size of the data stream (see`d`).<br>**Default:** command is ignored if a value is not specified|
|`d =` bytes per row|**Values:** 1 to 99999<br>This is the number of bytes in the downloaded data that comprise one row of<br>the image.<br>**Default:** command is ignored if a value is not specified|
|`data =` data|**Values:**<br>**ASCII hexadecimal data:** `00` to`FF`<br>A string of ASCII hexadecimal numbers, two digits per image byte. CR and LF<br>can be inserted as needed for readability. The number of two-digit number<br>pairs must match the above count. Any numbers sent after count is satisfied<br>are ignored. A comma in the data pads the current line with**00** (white space),<br>minimizing the data sent.`~DN` or any caret or tilde character prematurely<br>aborts the download.<br>**Binary data:** Strictly binary data is sent from the host. All control prefixes are<br>ignored until the total number of bytes needed for the graphic format is sent.|


215


ZPL Commands


**Example:** This example downloads 8,000 total bytes of data and places the graphic data at location
100,100 of the bitmap. The data sent to the printer is in ASCII form.

```
^FO100,100^GFA,8000,8000,80,ASCII data

```

**Example:** This example downloads 8,000 total bytes of data and places the graphic data at location
100,100 of the bitmap. The data sent to the printer is in binary form.

```
^FO100,100^GFB,8000,8000,80,Binary data

```

216
# ^BR




ZPL Commands


The `^BR` command is a barcode type for space-constrained identification from EAN International and the
Uniform Code Council, Inc.


**GS1 Databar (formerly Reduced Space Symbology)**

**Format:** `^BRa,b,c,d,e,f`











|Parameters|Details|
|---|---|
|`a =` orientation|**Values:**<br>`N =` Normal<br>`R =` Rotated<br>`I =` Inverted<br>`B =` Bottom-up<br>**Default:** `R`|
|`b =` symbology type in<br>the GS1 DataBar family|**Values:**<br>`1 =` GS1 DataBar Omnidirectional<br>`2 =` GS1 DataBar Truncated<br>`3 =` GS1 DataBar Stacked<br>`4 =` GS1 DataBar Stacked Omnidirectional<br>`5 =` GS1 DataBar Limited<br>`6 =` GS1 DataBar Expanded<br>`7 =` UPC-A<br>`8 =` UPC-E<br>`9 =` EAN-13<br>`10 =` EAN-8<br>`11 =` UCC/EAN-128 and CC-A/B`12 =` UCC/EAN-128 and CC-C<br>**Default:** `1`|
|`c =` magnification factor|**Values:** `1` to`10`<br>**Default:** 24 dot`= 6`, 12 dot is`3`, 8 dot and lower is`2`12 dot`= 6`, > 8 dot is`3`,<br>`8` dot and less is`2`|
|`d =` separator height|**Values:** `1` or`2`<br>**Default:** `1`|
|`e =` Barcode height|The bar code height only affects the linear portion of the barcode. Only<br>UCC/EAN and CC-A/B/C.<br>**Values:** `1` to`32000` dots<br>**Default:** `25`|
|`f =` the segment width<br>(GS1 DataBar Expanded<br>only)|**Values:** `2` to`22`, even numbers only, in segments per line<br>**Default:** `22`|


135


ZPL Commands


**Examples**


This is an example of Symbology Type 7 - UPC-A:


This is an example of Symbology Type 1 - GS1 DataBar Omnidirectional:


136
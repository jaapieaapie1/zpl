# ^FM




ZPL Commands


The `^FM` command allows you to control the placement of bar code symbols.


**Multiple Field Origin Locations**

It designates field locations for the PDF417 ( `^B7` ) and MicroPDF417 ( `^BF` ) bar codes when the structured
append capabilities are used. This allows printing multiple bar codes from the same set of text information.


The structured append capability is a way of extending the text printing capacity of both bar codes. If a
string extends beyond what the data limitations of the bar code are, it can be printed as a series: 1 of 3, 2 of
3, 3 of 3. Scanners read the information and reconcile it into the original, unsegmented text.

The `^FM` command triggers multiple bar code printing on the same label with `^B7` and `^BF` only. When
used with any other commands, it is ignored.

**Format:** `^FMx1,y1,x2,y2,...`






|Parameters|Details|
|---|---|
|`x1 =` x-axis location of<br>first symbol (in dots)|**Values:**<br>`0` to`32000e =` exclude this bar code from printing<br>**Default:** a value must be specified a value must be specified|
|`y1 =` y-axis location of<br>first symbol (in dots)|**Values:**<br>`0` to`32000e =` exclude this bar code from printing<br>**Default:** a value must be specified|
|`x2 =` x-axis location of<br>second symbol (in dots)|**Values:**<br>`0` to`32000e =` exclude this bar code from printing<br>**Default:** a value must be specified|
|`y2 =` y-axis location of<br>second symbol (in dots)|**Values:**<br>`0` to`32000e =` exclude this bar code from printing<br>**Default:** a value must be specified|
|`… =` continuation of X,Y<br>pairs|**Maximum number of pairs:** `60`|



**Example:** This example shows you how to generate three bar codes with the text “Zebra Technologies
Corporation strives to be…” would need to be repeated seven times, which includes 2870 characters of
data (including spaces) between `^FD` and `^FS` :


197


ZPL Commands


1 The ellipse is not part of the code. It indicates that the text needs to be repeated seven times,
as mentioned in the example description.


**Example:** This example assumes a maximum of three bar codes, with bar code 2 of 3 omitted:


198


ZPL Commands


**Comments:** Subsequent bar codes print once the data limitations of the previous bar code have been
exceeded. For example, bar code 2 of 3 prints once 1 of 3 has reached the maximum amount of data it can
hold. Specifying three fields does not ensure that three bar codes print; enough field data to fill three bar
code fields has to be provided.

The number of the `x,y` pairs can exceed the number of bar codes generated. However, if too few are
designated, no symbols print.


199
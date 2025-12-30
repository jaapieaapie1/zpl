# ^FE




ZPL Commands


The `^FE` command allows field data concatenation and substring extraction by referencing `^FN` fields.


**Field Concatenation**

The `^FE` command must precede each `^FD` command where it is used and only applies to that `^FD` field.
If a `^FE` does not immediately precede a `^FD`, then there is no field concatenation character active for that
`^FD` .

**Format:** `^FEa`

|Parameters|Details|
|---|---|
|`a`|**Values:**<br>Any character except the current format and control prefix (^ and ~<br>by default).<br>**Default:**` #`|



This delimiter, when defined, will be used to specify data from `^FN` fields, or parts thereof, that will be
inserted into a `^FD` field. For the following description, it is assumed that the `^FE` character is the default
character, `#` . There are two ways to insert data: including an entire `^FN` field and including part of a `^FN`
field.

To insert an entire `^FN` field, the syntax placed in the `^FD` field would be `#` n `#`, where n is the number of the
`^FN` . As an example:

```
^FN2^FDField FN 2 Data^FS
^FN3^FDField FN 3 Data^FS
^FE#^FD#2# and then #3#^FS

```

would result in the final `^FD` with the data `Field FN2 Data and then Field FN3 Data` .

To insert part of a `^FN` field, the syntax placed in the `^FD` field would be `#n,a,x,y#`, where:

|Parameters|Detail|
|---|---|
|`n`|The number of the`^FN` from which the data is taken.|
|`a` either`f` or`b`|`f` (for forward) indicates that data is taken from the from the start of<br>the`^FN`, and`b` (for backward) indicates that data is taken from the<br>end.|
|`x`|The starting position counts from the first or last character. 1<br>indicates start with the first character. If`a` is`b`, then x starts counting<br>from the end of the data and counts backward, so 1 would be<br>starting from the last character. 0 is invalid, and the field inserting<br>description is ignored, which is also the case for negative numbers|
|`y`|The number of characters to take. If y is greater than the number of<br>characters available, all the characters are taken from the starting<br>position to the last character.|



For the following examples, the first two lines of ZPL are assumed to be:

```
^FN2^FDField FN 2 Data^FS
^FN3^FDField FN 3 Data^FS

```

191


ZPL Commands


**Example 1**

```
^FE#^FD#2,f,1,5#^FS

```

Results in the data `Field` .


**Example 2**

```
^FE#^FD#2,f,7,4#^FS

```

results in the data `FN 2`


**Example 3**

```
^FE#^FD#2,b,1,4#^FS

```

Results in the data `Data`


**Example 4**

```
^FE#^FD#2# and #3,f,10,6#^FS

```

Results in the data `Field FN 2 Data and 3 Data`


**Supported Zebra Printers**


- ZD421C


- ZD421D


- ZD621D


- ZD621T


- ZT411


- ZT421


- ZT510


- ZT610


- ZT620


192
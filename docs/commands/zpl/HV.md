# ^HV




ZPL Commands


Use this command to return data from specified fields, along with an optional ASCII header, to the host
computer. You can use this command with any field that has been assigned a number with the `^FN` and
`^RF` commands.


**Host Verification**

**Format:** `^HV#,n,h,t,a`







|Parameters|Details|
|---|---|
|`#`  = field number<br>specified with another<br>command|The value assigned to this parameter should be the same as the one used<br>in another command.<br>**Values:** `0` to`9999`<br>**Default:** `0`|
|`n` = number of bytes to be<br>returned|**Values:** `1` to`256`<br>**Default:** `64`|
|`h` = header to be returned<br>with the data|Delimiter characters terminate the string. This field is Field Hex (`^FH`)<br>capable.<br>**Values:** `0` to`3072` bytes<br>**Default:** no header|
|`t` = termination|This field is Field Hex (`^FH`) capable.<br>**Values:** 0 to 3072 characters|
|`a` = command applies to|When`^PQ` is greater than 1 or if a void label occurs, send one response for<br>a label format or one for every label printed.<br>**Values:**<br>•<br>`F` = Format<br>•<br>`L` = Label<br>**Default:** `F`|


The following code:

```
^XA
.
.
.
^FH_^HV0,8,EPC[,]_0D_0A,L^FS
^PQ2
^XZ

```

Would return data similar to this:

```
EPC[12345678]
EPC[55554444]

```

239
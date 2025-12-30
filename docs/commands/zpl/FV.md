# ^FV




ZPL Commands


`^FV` replaces the `^FD` (field data) command in a label format when the field is variable.


**Field Variable**

**Format:** `^FVa`






|Parameters|Details|
|---|---|
|`a =` variable field data to<br>be printed|**Values:** `0` to`3072` byte string<br>**Default:** if no data is entered, the command is ignored|



**Example:** This is an example of how to use the `^MC` and `^FV` command:


**Comments:** `^FV` fields are always cleared after the label is printed. `^FD` fields are not cleared.


207
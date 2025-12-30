# ^PF




ZPL Commands


The `^PF` command causes the printer to slew labels (move labels at a high speed without printing) a
specified number of dot rows from the bottom of the label. This allows faster printing when the bottom
portion of a label is blank.


**Slew Given Number of Dot Rows**

**Format:** `^PF#`





|Parameters|Details|
|---|---|
|`# =` number of dots rows<br>to slew|**Values:** `0` to`32000`<br>**Default:** a value must be entered or the command is ignored.|


315
# ^FD




ZPL Commands


The `^FD` command defines the data string for a field. The field data can be any printable character except
those used as command prefixes ( `^` and `~` ).


**Field Data**


In RFID printers, it can also be used to specify passwords to write to tags.

**Format:** `^FDa`






|Parameters|Details|
|---|---|
|`a =`<br>•<br>data to be printed (all<br>printers), or<br>•<br>a password to be<br>written to a RFID tag<br>(rfid printers)|**Values:** any data string up to 3072 bytes<br>**Default:** none—a string of characters must be entered|



**Comments:** The `^` and `~` characters can be printed by changing the prefix characters—see ^CD ~CD on
page 153 and ^CT ~CT on page 166. The new prefix characters cannot be printed.

Characters with codes above 127, or the `^` and `~` characters, can be printed using the `^FH` and `^FD`
commands.

- `^CI13` must be selected to print a backslash (\).

For information on using soft hyphens, see ^FB on page 186.


190
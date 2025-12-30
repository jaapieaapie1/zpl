# ^FR




ZPL Commands


The `^FR` command allows a field to appear as white over black or black over white. When printing a field
and the `^FR` command has been used, the color of the output is the reverse of its background.


**Field Reverse Print**

**Format:** `^FR`

**Example:** In this example, the `^GB` command creates areas of black, allowing the printing to appear white:


**Comments:** The `^FR` command applies to only one field and has to be specified each time. When multiple
`^FR` commands are going to be used, it might be more convenient to use the `^LR` command.


203
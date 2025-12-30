# LOAD




ZBI Commands


This command transfers a program file previously stored in the printer’s memory and opens it in the ZBI
Program Memory.


If the program file does not exist, the ZBI Program Memory is cleared and no program is opened.


**Format**
```
   LOAD <filename$>
```

**Parameters**
`<filename$>` = the file name to be loaded into memory. Drive location and file name must be in
quotation marks. If the drive location is not specified, all drives will be searched.

**Example**
Here are examples of how to use the LOAD command:

```
   LOAD "PROGRAM1.BAS"
   LOAD "E:PROGRAM1.BAS"

```

**Comments**
This is an interactive command that takes effect as soon as it is received by the printer.


511
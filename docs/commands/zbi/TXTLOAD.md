# TXTLOAD



This function will read the contents of an ASCII text file into a ZBI string variable.


**Format**

```
          TXTLOAD(DEST$, FILENAME$)

```

**Parameters**

              - `DEST$` = string to store the contents of `FILENAME$` .

              - `FILENAME$` = name of the file to read. Drive location and file name must be in quotation marks.
The file extension must be either ". `CSV` " or ". `TXT` ".

**Returns**
The number of bytes read from the file. The function will return 0 if the file could not be read.

**Example**
This example shows how to print out the contents of a file.

```
          10 LET TXTSIZE = TXTLOAD(TXTDATA$, "E:MYDATA.TXT")
          20 PRINT STR$(TXTSIZE), "bytes:", TXTDATA$

```

**Comments**
The data originally in `DEST$` will be overwritten upon completion of this function.


518


ZBI Commands
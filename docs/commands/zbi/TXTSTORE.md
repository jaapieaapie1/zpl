# TXTSTORE



This function will store the contents of a ZBI string in an ASCII text file.


**Format**
```
          TXTSTORE(SRC$, FILENAME$)
```

**Parameters**

              - `SRC$` = string to store to `FILENAME$` .

              - `FILENAME$` = name of the file to store. Drive location and file name must be in quotation marks.
The file extension must be either ". `CSV` " or ". `TXT` ".

**Returns**
Returns a 0 if there were no errors, otherwise a 1 is returned.

**Example**


This example shows how to append a text file.

```
          10 LET TXTSIZE = TXTLOAD(TXTDATA$, "E:MYDATA.TXT")
          11 REM Append a date/time stamp to the file
          20 LET TXTDATA$ = TXTDATA$ & " " & DATE$ & " " & TIME$
          30 LET TXTSIZE = TXTSTORE(TXTDATA$, "E:MYDATA.TXT")
          40 PRINT TXTDATA$

```

519
# CSVSTORE



This function will store the values of a two dimensional array into a CSV file on the file system. Each
element within the array is treated as a single value within the CSV file.


**Format**
```
          CSVSTORE(SRC$, FILENAME$)
          CSVSTORE(SRC$, FILENAME$, DELIM$)
```

**Parameters**

              - `SRC$` = two dimensional array of strings to be written to a CSV file.

              - `FILENAME$` = name of the file to store the array contents. Drive location and file name must be
in quotation marks. The file extension must be either ". `CSV` " or ". `TXT` ".

              - `DELIM$` = optional delimiter that is used in the CSV file instead of a comma. If DELIM$ is not
provided a comma will be used by default. The delimiter must be a single character that is not a
quote, carriage return, or newline.


**Returns**
A 0 if there were no errors. A 1 is returned if `SRC$` is not a string array, if the file could not be
written, or if `SRC$` contains errors that prevent the file from being stored.

**Example**
This example shows how to convert a comma delimited CSV file into a "^" delimited TXT file and
print the contents.

```
          10 DECLARE STRING CSVDB$(1,2)
          20 LET NUMOFCOLS = CSVLOAD(CSVDB$, "E:RECORDS.CSV")
          30 LET CSVERROR = CSVSTORE(CSVDB$, "E:NEWREC.TXT", "^")
          40 LET NUMOFCOLS = CSVLOAD(CSVDB$, "E:NEWREC.TXT", "^")
          50 LET NUMOFROWS = ROWSIZE(CSVDB$)
          100 FOR I = 1 TO NUMOFROWS STEP 1
          110   FOR J = 1 TO NUMOFCOLS STEP 1
          120     PRINT CSVDB$(I, J), " ";
          200   NEXT J
          210   PRINT ""
          300 NEXT I

```

**Comments**
[The elements of the array should follow the rules in IETF RFC 4180: http://tools.ietf.org/html/rfc4180](http://tools.ietf.org/html/rfc4180)


There is no limit on the number of columns per row when storing to a CSV file. However, a file
stored with rows that exceed the column limit imposed by `CSVLOAD` will not be loaded by the
`CSVLOAD` function.

There is no limit on the size of a row when stored to a CSV file. However, a file stored with rows
that exceed the size limit imposed by `CSVLOAD` will not be loaded by the `CSVLOAD` function.


517


ZBI Commands
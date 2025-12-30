# CSVLOAD



This function will load the delimited values from a CSV file, defined by `FILENAME$`, and store them in the
two-dimensional array, `DEST$` .


**Format**
```
          CSVLOAD(DEST$, FILENAME$)
          CSVLOAD(DEST$, FILENAME$, DELIM$)
```

**Parameters**
`DEST$` = two dimensional array that will hold the rows and columns from the CSV file specified by
the `FILENAME$` variable. If there is not enough room in `DEST$`, or if it has the wrong size, it will be
changed to fit the data from the file. The data originally in `DEST$` will be overwritten.

`FILENAME$` = name of the file to load. Drive location and file name must be in quotation marks. The
file extension must be either " `.CSV` " or " `.TXT` ".

`DELIM$` = optional delimiter that is used in the CSV file instead of a comma. If `DELIM$` is not
provided a comma will be used by default. The delimiter must be a single character that is not a
quote, carriage return, or newline.


**Returns**
The number of elements in each row of the CSV file. The function will return 0 if errors were
detected in the CSV file, or if the file could not be read.

**Example**
This example shows how to print the values in a CSV file with a comma delimiter.

```
          10 DECLARE STRING CSVDB$(1,2)
          20 LET FILENAME$ = "E:RECORDS.CSV"
          30 LET NUMOFCOLS = CSVLOAD(CSVDB$, FILENAME$)
          40 LET NUMOFROWS = ROWSIZE(CSVDB$)
          100 FOR I = 1 TO NUMOFROWS STEP 1
          110   FOR J = 1 TO NUMOFCOLS STEP 1
          120     PRINT CSVDB$(I, J), " ";
          200   NEXT J
          210   PRINT ""
          300 NEXT I

```

515


ZBI Commands


**Example**
This example shows how to print the values in a CSV file that uses a '|' as a delimiter.

```
          10 DECLARE STRING CSVDB$(1,2)
          20 LET FILENAME$ = "E:EMPLOYEE.CSV"
          30 LET NUMOFCOLS = CSVLOAD(CSVDB$, FILENAME$, "|")
          40 LET NUMOFROWS = ROWSIZE(CSVDB$)
          100 FOR I = 1 TO NUMOFROWS STEP 1
          110   FOR J = 1 TO NUMOFCOLS STEP 1
          120     PRINT CSVDB$(I, J), " ";
          200   NEXT J
          210   PRINT ""
          300 NEXT I

```

**Comments**
The maximum CSV file size supported will vary based upon available RAM within the printer.
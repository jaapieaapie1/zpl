# COLUMNSIZE



This function will return the number of columns in an array.


**Format**
COLUMNSIZE(A)


COLUMNSIZE(A$)


**Parameters**
A = integer array to query for the number of columns.


A$ = string array to query for the number of columns.


**Returns**
A 0 if the variable is not an array. Returns 1 if the array has only one dimension. Returns the size of
the second dimension if the array has two dimensions.

**Example**
This example shows how to determine the number of elements in a one dimensional string array.

```
          10 DECLARE STRING NAMES$(3)
          20 LET NAMES$(1) = "Fred"
          30 LET NAMES$(2) = "Wilma"
          40 LET NAMES$(3) = "Barney"
          50 REDIM NAMES$(4) ! Make room for Betty
          60 LET NAMES$(4) = "Betty"
          70 LET NUMOFCOLS = COLUMNSIZE(NAMES$)
          80 PRINT NUMOFCOLS

```

**Example**
This example shows how to determine the number of columns in a two dimensional numeric array.

```
          10 DECLARE NUMERIC SQROFTWOLOOKUP(3,2)
          20 LET SQROFTWOLOOKUP (1,1) = 1
          30 LET SQROFTWOLOOKUP (1,2) = 2
          40 LET SQROFTWOLOOKUP (2,1) = 2
          50 LET SQROFTWOLOOKUP (2,2) = 4
          60 LET SQROFTWOLOOKUP (3,1) = 3
          70 LET SQROFTWOLOOKUP (3,2) = 8
          80 LET COLCNT = COLUMNSIZE(SQROFTWOLOOKUP)
          90 PRINT COLCNT

```

575
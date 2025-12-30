# DELROW




ZBI Commands


This command will delete a row from an existing array.


**Format**
```
   DELROW (<ARRAYNAME>, <INDEX>)
```

**Parameters**
`<ARRAYNAME>` = the array where the row will be deleted

`<INDEX>` = index of the row to delete from the array

**Example**
This example shows how to delete a row from the middle of an array.

```
   10 DECLARE NUMERIC SCORES(5)
   20 LET SCORES(1) = 85
   30 LET SCORES(2) = 92
   40 LET SCORES(3) = 98
   50 LET SCORES(4) = 45
   60 LET SCORES(5) = 100
   70 DELROW(SCORES, 4) ! Remove the low score

```

**Comments**
This decreases the size of A by one row, and moves all the rows from INDEX to the end of the array
down by one, overwriting the row at position INDEX.

`INDEX` cannot be any larger the number of rows in the array.

If the array only has one row, that row may not be deleted.


This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.


573
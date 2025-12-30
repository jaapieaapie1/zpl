# INSERTROW



This command will insert a new row into an existing array.


**Format**
```
          INSERTROW (<ARRAYNAME>, <INDEX>)
```

**Parameters**
`<ARRAYNAME>` = array where the row will be inserted

`<INDEX>` = index of the row in the array that the new row will be inserted before

**Example**
This example shows how to insert a row into the middle of an array.

```
          10 DECLARE NUMERIC SCORES(3)
          20 LET SCORES(1) = 85
          30 LET SCORES(2) = 92
          40 LET SCORES(3) = 98
          50 INSERTROW(SCORES, 2)
          60 LET SCORES(2) = 100

```

**Example**
This example shows how to add a row into the end of an array.

```
          10 DECLARE NUMERIC SCORES(3)
          20 LET SCORES(1) = 85
          30 LET SCORES(2) = 92
          40 LET SCORES(3) = 98
          50 INSERTROW(SCORES, 4)
          60 LET SCORES(4) = 100

```

**Comments**
Inserting a row increases the size of the array by one row, and moves all the rows from `INDEX` to
the end of the array up one row, leaving an empty row at position `INDEX` .

`INDEX` cannot be any larger the number of rows in the array plus one. If the number of rows plus
one is provided, the new row will be added to the end of the array.


This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.


572
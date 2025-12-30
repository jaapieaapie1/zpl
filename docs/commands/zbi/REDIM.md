# REDIM




ZBI Commands


This command will change the dimensions of an array.


**Format**

```
   REDIM <ARRAYNAME>(<SIZE>)
   REDIM <ARRAYNAME>(<ROWS>,<COLUMNS>)
   REDIM <ARRAYNAME$>(<SIZE>)
   REDIM <ARRAYNAME$>(<ROWS>,<COLUMNS>)

```

**Parameters**
<SIZE> = new number of entries in a single dimension array.


<ROWS> = new number of rows in a two dimensional array.


<ROWS> = new number of rows in a two dimensional array.


<COLUMNS> = new number of columns in a two dimensional array.


**Example**
This example shows how to change a one dimensional numeric array.

**Example**
This example shows how to change a two dimensional string array.

```
   10 DECLARE STRING NAMEAGES$(3,2)
   20 LET NAMEAGES$(1,1) = "Abraham"
   30 LET NAMEAGES$(1,2) = "Lincoln"
   40 LET NAMEAGES$(2,1) = "Dwight"
   50 LET NAMEAGES$(2,2) = "Eisenhower"
   60 LET NAMEAGES$(3,1) = "Theodore"
   70 LET NAMEAGES$(3,2) = "Roosevelt"
   80 REDIM NAMEAGES$(5,2) ! Make room for more

```

**Comments**
The `REDIM` must have the same number of dimensions as the original declaration of the array.

    - If the array has two dimensions, the second array bound cannot change. It must have the same
value as the original declaration.

    - If `REDIM` makes an array smaller, elements (or rows, for a two dimensional array) at the end of
the array are discarded.

    - If `REDIM` makes an array larger, elements (or rows) are added at the end of the array, and
initialized as they would be with a `DECLARE` .

This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.


571


ZBI Commands
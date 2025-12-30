# FIND




ZBI Commands


This function will find an element of a string array that contains an identified search string.


**Format**
FIND(A$, B$)


FIND(A$, B$, START)


FIND(A$, COLUMN, B$)


FIND(A$, COLUMN, B$, START)


**Parameters**


    - A$ = string array to search for B$.


    - B$ = string to search for within A$.


    - START = index within a single dimensional array, or row for a two dimensional array, to start the
search.


    - COLUMN = column to isolate search to in a two dimensional array. This must be supplied if A$ is
a two dimensional array.


**Returns**
Returns a 0 if B$ is not found or if there was an error. Otherwise, returns the index that contains
the first occurrence of the string B$ (the element index for one dimensional arrays, the row for two
dimensional arrays).

**Example**
This example shows how to find a string in a one dimensional array.

```
   10 DECLARE STRING NAMES$(4)
   20 LET NAMES$(1) = "Fred"
   30 LET NAMES$(2) = "Wilma"
   40 LET NAMES$(3) = "Barney"
   50 LET NAMES$(4) = "Betty"
   60 LET BARNEYIX = FIND(NAMES$, "Bar")
   70 PRINT "Found Barney in element "; STR$(BARNEYIX)

```

576


ZBI Commands


**Example**
This example shows how to find a string that occurs more than once in a two dimensional array.

```
 10 DECLARE STRING CLOTHING$(5,2)
 20 LET TYPECOL   = 1
 30 LET MATERIALCOL = 2
 40 LET CLOTHING$(1,1) = "Gloves"
 50 LET CLOTHING$(1,2) = "Knit"
 60 LET CLOTHING$(2,1) = "Pants"
 70 LET CLOTHING$(2,2) = "Cotton"
 80 LET CLOTHING$(3,1) = "Gloves"
 90 LET CLOTHING$(3,2) = "Leather"
 100 LET CLOTHING$(4,2) = "Shirts"
 110 LET CLOTHING$(4,2) = "Polyester"
 120 LET CLOTHING$(5,2) = "Pants"
 130 LET CLOTHING$(5,2) = "Denim"
 140 LET GLOVEIX = 1
 150 DO
 160 LET GLOVEIX = FIND(CLOTHING$, TYPECOL, "Gloves", GLOVEIX)
 170 IF NOT GLOVEIX = 0 THEN
 180 PRINT CLOTHING$(GLOVEIX, MATERIALCOL), "gloves are available"
 190 LET GLOVEIX = GLOVEIX + 1
 200 END IF
 210 LOOP WHILE NOT GLOVEIX = 0

```

**Comments**
`COLUMN` must be greater than 0.

If START is given, it must be greater than 0.


FIND will match the first occurrence of B$, even if it is a substring of a string within the A$ array. For
example, "Coat" will be found in both locations 1 and 4.

```
 5 DECLARE STRING A$(5)
 10 LET A$(1) = "Over Coat"
 20 LET A$(2) = "Hat"
 30 LET A$(3) = "Jacket"
 40 LET A$(4) = "Coat"
 50 LET A$(5) = "Boots"

```

If an exact match is needed, `FIND` should be called until 0 is returned or the item is found and
confirmed. To confirm, check the item against the expected item, it should match exactly. See CSV
Program on page 587 for an example showing how to do this.


577


ZBI Commands
# SEARCHTO$



This function performs a search until a specified string is found. The string the search yields is displayed.


**Format**
```
          SEARCHTO$(A,B$)
          SEARCHTO$(A,B$,C)
          SEARCHTO$(A$,B$)
          SEARCHTO$(A$,B$,C$)
```

**Parameters**

              - `A` = port number (0 to 9) to which requested data is sent

              - `A$` = string to search for B$

              - `B$` = string variable or string array. If B$ is an array, this command searches for all
non-null
strings in the B$ array.

              - `C` = a port in which the input is directed until B$ is found

              - `C$` = a string in which the characters in A$ are directed until B$ is found

**Returns**
The string found.

**Example**
This example shows how to use `SEARCHTO` to find a string on a port:

```
          10 OPEN #1: NAME "SER"
          20 LET A$ = SEARCHTO$(1,"^XA")
          30 PRINT "FOUND:", A$

```

**Example**
This example shows how to search for an array of strings:

```
          10 OPEN #1: NAME "SER"
          20 DECLARE STRING FIND$(3)
          30 LET FIND$(1) = "ONE"
          40 LET FIND$(2) = "TWO"
          50 LET FIND$(3) = "THREE"
          60 LET A$ = SEARCHTO$(1,FIND$)
          70 PRINT "FOUND:", A$

```

501


ZBI Commands


**Example**
This example shows unused data routed to a port.

```
 10 OPEN #1: NAME "PAR"
 20 OPEN #2: NAME "SER"
 30 DECLARE STRING FIND$(3)
 40 LET FIND$(1) = "ONE"
 50 LET FIND$(2) = "TWO"
 60 LET FIND$(3) = "THREE"
 70 LET A$ = SEARCHTO$(1,FIND$,2)
 80 PRINT "FOUND:", A$

```

**Example**
This example shows how to use `SEARCHTO` to find a string within a string and direct the unused
part of the string to another string:

```
 10 LET A$ = "The faster you
 go, the shorter you are - Einstein"
 20 LET B$ = SEARCHTO$(A$,"you", C$)
 30 PRINT "FOUND:", B$
 40 PRINT "DISCARDED:", C$

```

**Comments**
`SEARCHTO` will block (wait) until the search string is found. If you want to be able to run other code
while doing something similar, consider using `READ` with `POS` .

When using `SEARCHTO` with ports, it will block (wait) until the search string is found. If you want to
be able to run other code while doing something similar, consider using `READ` to place data into a
string. That string can be passed to `SEARCHTO` for processing.


502


ZBI Commands
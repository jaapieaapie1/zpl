# POS




ZBI Commands


This function returns the location of the first occurrence of a search string in the target string. It can be
assigned an index.


**Format**
```
   POS(A$,B$)
   POS(A$,B$,M)
```

**Parameters**

    - `A$` = the target string to search

    - `B$` = the search string to find in `A$`

    - `M` = The index to start looking for `B$` . If omitted, the search will start at the beginning of the
string. `M` must be greater than zero.

**Returns**
The location of the string. If the string is not found, this will return 0.

**Example**
This is an example of how to use the POS command:

```
   10 LET A$="Hello World"
   20 LET B$="o"
   30 PRINT POS(A$,B$)
   40 PRINT POS(A$,B$,1)
   50 PRINT POS(A$,B$,6)
   RUN
   5
   5
   8

```

**Comments**


None


557
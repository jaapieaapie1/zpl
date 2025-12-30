# VAL




ZBI Commands


This function evaluates the number represented by a string.


**Format**
```
   VAL(A$)
```

**Parameters**
`A$` = This is the input string to pull the number from. Non-numbers are ignored.

**Returns**
The numeric representation of the string.

**Example**
This is an example of how to use the VAL(A$)command:

```
   10 LET A$="123"
   20 LET C=VAL(A$)
   30 PRINT C
   RUN
   123

   PRINT VAL("321A123")
   321123

```

**Comments**
None


565


ZBI Commands
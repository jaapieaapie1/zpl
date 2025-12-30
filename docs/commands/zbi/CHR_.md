# CHR$




ZBI Commands


This function takes a value between 0 and 255 and puts that value into a string.


**Format**
```
   CHR$(VAL)
```

**Parameters**
`(VAL)` = The numeric value of the string character.

**Returns**
A single character string containing the value entered.

**Example**


This is an example of how to use the CHR$ command to easily put control characters into strings:

```
   10 LET NULL$=CHR$(0)
   20 LET STX$=CHR$(2)
   30 LET ETX$=CHR$(3)
   40 LET EOT$=CHR$(4)

```

**Comments**
None


544
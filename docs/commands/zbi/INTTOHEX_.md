# INTTOHEX$



This function will take a numeric value and convert it into a hexadecimal string. The range of values for
integers is: -2,147,483,648 to +2,147,483,647


**Format**
```
          INTTOHEX$(A)
```

**Parameters**
`A` = The numeric value to convert.

**Returns**
A string representing the integer in hex.


566


ZBI Commands


**Example**
These print statements show the output of the `INTTOHEX$` function given different values. These
print statements show the output of the `INTTOHEX$` function given different values.

```
 PRINT INTTOHEX$(1)
 1

 PRINT INTTOHEX$(10)
 A

 PRINT INTTOHEX$(16)
 10

 PRINT INTTOHEX$(20)
 14

 PRINT INTTOHEX$(30)
 1E

 PRINT INTTOHEX$(100)
 64

 PRINT INTTOHEX$(123124)
 1EOF4

 PRINT INTTOHEX$(-5)
 0

 PRINT INTTOHEX$(-99)
 0

```

**Comments**
Negative values will be returned as 0.


567


ZBI Commands
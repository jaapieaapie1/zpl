# HEXTOINT



This function will convert hexadecimal strings to integers.


**Format**
HEXTOINT `(A$)`

**Parameters**
`A$` = The hex string to convert.

**Returns**
A integer string computed from the hexadecimal string.

**Example**
These print statements show the output of the `INTTOHEX` function given different values.

```
          PRINT HEXTOINT("0")
          0

          PRINT HEXTOINT("A")
          10

          PRINT HEXTOINT("a")
          10

          PRINT HEXTOINT("1A")
          26

          PRINT HEXTOINT("10")
          16

          PRINT HEXTOINT("AaAa")
          43690

          PRINT HEXTOINT("AAAA")
          43690

          PRINT HEXTOINT("-1")
          0

          PRINT HEXTOINT("-A")
          0

```

568


ZBI Commands


**Comments**
Negative values will be returned as 0.


569


ZBI Commands
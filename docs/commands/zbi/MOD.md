# MOD




ZBI Commands


This function computes the remainder from division. (This is known as the modulus.)


**Format**
```
   MOD(X,Y)
```

**Parameters**
`X` = the value to be modulated (numerator).

`Y` = the base number or divisor (denominator).

**Returns**
The remainder of the division (X/Y).

**Example**
This is an example of how to use the MOD(X,Y)command:

```
   10 PRINT MOD(25,10)
   20 PRINT MOD(2,1)
   30 PRINT MOD(3,2)
   40 PRINT MOD(9,2)
   50 PRINT MOD(-2,9)
   60 PRINT MOD(2,0)
   RUN
   5
   0
   1
   1
   -2
   ERROR OCCURRED ON LINE 60:DIVIDE BY ZERO

```

**Comments**
None


564
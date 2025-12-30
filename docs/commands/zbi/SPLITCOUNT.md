# SPLITCOUNT



This function returns the number of sub-strings that would be returned by the SPLIT function.


**Format**
```
          SPLITCOUNT(SOURCE$, DELIMITER$)
```

**Parameters**
`SOURCE$` = the string that will be searched for the provided delimiter.

`DELIMITER$` =5

**Returns**
The number of sub-strings that would be returned by the `SPLITCOUNT` function.

**Example**

This function shows how to determine the number of sub-strings that the `SPLITCOUNT` command
would produce

```
          10 LET CNT = SPLITCOUNT("ONE,,,FOUR,FIVE,,SEVEN,", ",")
          20 PRINT "Number of sub-strings returned is", STR$(CNT)
          RUN
          Number of sub-strings returned is 8

```

**Comments**
None


552
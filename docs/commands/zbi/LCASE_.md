# LCASE$




ZBI Commands


This function will convert a string to all lowercase characters.


**Format**
```
   LCASE$ (A$)
```

**Parameters**
`(A$)` = the string that will be converted

**Returns**
The characters in `A$` converted to lowercase.

**Example**

This is an example of how to use the `LCASE$` command.

```
   10 LET B$=LCASE$ ("Hello World")
   20 PRINT B$
   RUN
   hello world

```

**Comments**
This will only work on non-accented Latin characters, A-Z.


543
# RTRIM$




ZBI Commands


This function returns a string with trailing spaces removed.


**Format**
```
   RTRIM$(A$)
```

**Parameters**
`(A$)` = the base string

**Returns**
`A$` with trailing spaces removed.

**Example**


This is an example of how to use the RTRIM$(A$)command:

```
   10 LET A$="Hello "
   20 LET B$="World"
   30 PRINT A$ & B$
   40 PRINT RTRIM$(A$)& B$
   RUN
   Hello World
   HelloWorld

```

**Comments**
None


547
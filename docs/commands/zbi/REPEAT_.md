# REPEAT$




ZBI Commands


This function creates multiple copies of a string combined into a new string.


**Format**
```
   REPEAT$(A$,M)
```

**Parameters**

    - `A$` = the base string to duplicate

    - `M` = the number of times to duplicate `A$`

**Returns**
A string containing `M` copies of `A$` . Note: When M=0, an empty string is returned.

**Example**


This is an example of how to use the REPEAT$(A$,M)command:

```
   10 PRINT REPEAT$("Hello",3)
   RUN
   HelloHelloHello

```

**Comments**
None


546
# UCASE$




ZBI Commands


This function converts a string to all uppercase characters.


**Format**
```
   UCASE$(A$)
```

**Parameters**
`A$` = the base string to convert

**Returns**
`A$` converted to uppercase.

**Example**


This is an example of how to use the UCASE$(A$)command:

```
   10 LET A$="Zebra Technologies"
   20 PRINT UCASE$(A$)
   RUN
   ZEBRA TECHNOLOGIES

```

**Example**


This is an example of how to capitalize a line.

```
   10 LET A$="The Cow jUmped Over THE Moon."
   20 LET A$=LCASE$(A$)
   30 LET A$(1:1)=UCASE$(A$(1:1))
   40 PRINT A$
   RUN
   The cow jumped over the moon.

```

**Comments**
This will only convert non-accented Latin characters, a-z.


553


ZBI Commands
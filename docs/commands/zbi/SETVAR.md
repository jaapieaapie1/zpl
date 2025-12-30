# SETVAR




ZBI Commands


`SETVAR` allows the direct setting of printer parameters.


**Format**
```
   SETVAR (PARAM$, VALUE$)
```

**Parameters**
`PARAM$` = The printer parameter to set.

`VALUE$` = the value to set

**Returns**
Parameter dependent.

**Example**
This is an example of the `SETVAR` command:

```
   AUTONUM 1,1
   LET OUTSTR$ = "Processing"
   LET LOOPCTR = 200
   LET TIMER5 = 17
   LET TMP = REGISTEREVENT(TIMER5, 0, 1000)
   DO WHILE LOOPCTR > 0
   LET EVT = HANDLEEVENT()
   IF EVT = TIMER5 THEN
   LET A = SETVAR("device.frontpanel.line2",OUTSTR$)
   LET OUTSTR$ = OUTSTR$ & "."
   IF LEN(OUTSTR$) >16 THEN
   LET OUTSTR$ = "Processing"
   END IF
   END IF
   LET LOOPCTR = LOOPCTR - 1
   SLEEP 1
   LOOP
   LET TMP = UNREGISTEREVENT(TIMER5)
   LET A = SETVAR("device.frontpanel.line2","")
   END

```

**Comments**
None


584


ZBI Commands
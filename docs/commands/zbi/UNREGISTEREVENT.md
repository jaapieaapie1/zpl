# UNREGISTEREVENT



This function allows events that are currently set to be captured by the program to no longer be captured.
Once called events will return to the normal method of processing if the REGISTEREVENT function Z
parameter was set to 1.


**Format**
```
          UNREGISTEREVENT(X)
```

**Parameters**
`(X)` = the ID of the event to stop

**Returns**
0 if the event is a valid event to unregister. A -1 if the event does not exist.

**Example**
Here is an example of how to use the UNREGISTEREVENT command:

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


527


ZBI Commands
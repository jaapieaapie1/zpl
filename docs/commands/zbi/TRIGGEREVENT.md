# TRIGGEREVENT



This function allows for front panel buttons to be triggered programatically.


**Format**
```
          TRIGGEREVENT(X)
```

**Parameters**
`X` = the ID of the event from the possible event list to TRIGGER.

See the following printer tables for events that can be triggered by this command:


              - Xi4/RXi4/XiIIIPlus/PAX4/105SL/ZE500 on page 523


              - ZM400/ZM600/RZ400/RZ600/Z4Mplus/Z6Mplus on page 523


              - S4M on page 524


**Returns**
Always returns 0.

**Example**
Here are examples of how to use the `TRIGGEREVENT` command:

```
          1 REM THIS IS AN EXAMPLE OF HOW TO TRIGGER AN EVENT
          AUTONUM 1,1
          LET PAUSEKEY = 2
          DO WHILE 1 = 1
          LET A = TRIGGEREVENT(PAUSEKEY)
          LET A = SETVAR("device.frontpanel.line2",str$(A))
          SLEEP 2
          LOOP

```

**Comments**
None
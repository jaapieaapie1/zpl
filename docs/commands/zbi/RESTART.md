# RESTART




ZBI Commands


If a program was halted by a break point or the `BREAK` command, the `RESTART` command can be used to
reactivate the program at the point it stopped.

`RESTART` functions similar to `RUN`, except the program attempts to restart from the point where it was last
terminated. It also works in conjunction with the `STEP` command, picking up where the `STEP` command
ended.


**Format**
```
   RESTART
```

**Parameters**
N/A

**Example**
An example of the `RESTART` command:

```
   10 PRINT "TEN"
   20 PRINT "TWENTY"
   30 PRINT "THIRTY"
   RUN
   TEN
   TWENTY
   THIRTY

   STEP
   TEN

   RESTART
   TWENTY
   THIRTY

   ADDBREAK 20
   RUN
   TEN
   <Program Break> on line: 20

   DEBUG ON
   TRACE ON
   RESTART
   <TRACE> 20
   TWENTY
   <TRACE> 30
   THIRTY

```

**Comments**
If the program has not been run or has finished, `RESTART` runs the program from the beginning.

This is an interactive command that takes effect as soon as it is received by the printer.


459
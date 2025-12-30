# BREAK




ZBI Commands


This command allows you to stop the program when the program reaches this line.


**Format**
```
   BREAK
```

**Parameters**
N/A

**Example**
An example of BREAK command in use:

```
   10 LET A=5
   20 BREAK
   30 PRINT A
   DEBUG ON
   TRACE ON
   RUN
   <TRACE> 10
   <TRACE> A=5
   <TRACE> 20
   <USER BREAK>

```

**Comments**
This command is available only when the `DEBUG` function has been activated. When `DEBUG` is on,
`BREAK` halts processing. `RUN` starts the program from the beginning. `RESTART` allows the program
to continue from where it left off.


When using ZBI-Developer, this command will interfere with the debugging operations built into the
application.


This is a program command that must be preceded by a line number.


463


ZBI Commands
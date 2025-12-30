# ADDBREAK



This command allows you to stop the program when the program reaches a specified line.


**Format**
```
          ADDBREAK <A>
```

**Parameters**
`A` = the line number to break on. If the number specified is not in the program, the program will not
break.

**Example**
An example of the ADDBREAK command.

```
          10 LET A=5
          20 PRINT A
          ADDBREAK 20
          RUN
          <PROGRAM BREAK> ON LINE:20

          RESTART
          5

```

**Comments**
This command is available only when the DEBUG function has been activated. When DEBUG is on,
BREAK halts processing. RUN starts the program from the beginning. RESTART allows the program
to continue from where it left off.


This is the command used internally by ZBI-Developer when the user right-clicks over a program
line and adds a Breakpoint via the "Toggle Breakpoint" selection.


It is the recommended method for setting breakpoints in ZBI.


A maximum of 16 breakpoints can be set in an application.


This is an interactive command that takes effect as soon as it is received by the printer.


464


ZBI Commands
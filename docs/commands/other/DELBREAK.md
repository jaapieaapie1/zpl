# DELBREAK



This command allows you to remove existing breakpoints.


**Format**
```
          DELBREAK <A>
```

**Parameters**
`A` = the line number from which to remove the break. If 0 is specified, all break points will be
removed. If the number specified is not a breakpoint, the command will have no effect.

**Example**
An example of the DELBREAK command:

```
          10 LET A=5
          20 PRINT A
          ADDBREAK 20
          DEBUG ON
          TRACE ON
          RUN
          <TRACE> 10
          <TRACE> A=5
          <PROGRAM BREAK> ON LINE:20

          RESTART
          <TRACE> 20
          5

          DELBREAK 20
          RUN
          <TRACE> 10
          <TRACE> A=5
          <TRACE> 20
          5

```

**Comments**
This command is available only when the DEBUG function has been activated. When DEBUG is
on, BREAK halts processing, RUN starts the program from the beginning, and RESTART allows the
program to continue where it left off.


This is the command used internally by ZBI-Developer when the user right-clicks over a program
line and removes a Breakpoint via the "Toggle Breakpoint" selection.


A maximum of 16 breakpoints can be set in an application.


This is an interactive command that takes effect as soon as it is received by the printer.


465
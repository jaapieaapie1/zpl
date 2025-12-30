# RENUM




ZBI Commands


This command renumbers the lines of the program being edited. `RENUM` can reorganize code when line
numbers become over- or under-spaced. The line references following `GOTO` and `GOSUB` statements are
renumbered if they are constant numeric values. Renumbering does not occur if the line numbers are
outside of the range limits of 1 to 10000.


**Format**
```
   RENUM <A>,<B>
```

**Parameters**

`<A>` = the number to start the renumbering sequence

`<B>` = the automatic increment between the new line numbers

**Example**


This is an example of how to use the RENUM command:

```
   LIST
   13 LET A=6
   15 LET B=10
   17 GOTO 13
   RENUM 10,5
   LIST
   10 LET A=6
   15 LET B=10
   20 GOTO 10

```

**NOTE:** The target of the GOTO command changes from 13 to 10 to reflect the
renumbering.

**Comments**


This is an interactive command that takes effect as soon as it is received by the printer.


453
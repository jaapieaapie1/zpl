# AUTONUM



This command automatically generates sequential program line numbers.


**Format**
```
          AUTONUM <A>,<B>
```

**Parameters**
`A` = the number used to start the auto-numbering sequence

`B` = the automatic increment between the new line numbers

**Example**
This example shows specifying the starting line number in the increment between new line number.
Type the following at the prompt:

```
          AUTONUM 10,5
          SUB START
          PRINT "HELLO WORLD"
          GOTO START

          LIST

```

Will produce:

```
          AUTONUM 10,5
          10 SUB START
          15 PRINT "HELLO WORLD"
          20 GOTO START

```

The three lines are automatically started with the `AUTONUM` parameters; in this case, the first line
starts with 10 and each subsequent line increments by 5.


**Comments**
This feature is disabled by overwriting the current line number and entering the desired interactive
mode commands, or leaving the line blank.

Use of the `SUB` command allows for `GOTO` and `GOSUB` statements that do not require line numbers
in your program.


This is an interactive command that takes effect as soon as it is received by the printer.


452
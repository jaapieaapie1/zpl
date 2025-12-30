# EXIT




ZBI Commands


This command is used to exit the `DO` and `FOR` loops.

**Format**
```
   EXIT DO
   EXIT FOR
```

**Parameters**
The specified loop type is exited. For the `DO` command, the program will continue execution on the
line following the next `LOOP` . Likewise for the `FOR` command, the program will continue on the line
after the next `NEXT` command `.`

**Example**
N/A

**Comments**
This is a program command that is preceded by a line number. To be explicit and reduce errors, it is
recommended to use `GOTO` instead of `EXIT` .


482
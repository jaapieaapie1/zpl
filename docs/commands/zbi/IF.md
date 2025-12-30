# IF Statements



If the value of the `<Boolean expression>` in an `IF` statement is true and a program line follows the
keyword `THEN`, this program line is executed. If the value of the Boolean expression is false and a program
line follows the keyword `ELSE`, this program line is executed. If `ELSE` is not present, then execution
continues in sequence, with the line following the `END` `IF` statement.

Nesting of blocks is permitted, subject to the same nesting constraints as `DO-LOOP` s (no overlapping
blocks).

`ELSE IF` statements are treated as an `ELSE` line followed by an `IF` line, with the exception that the `ELSE`
`IF` shares the `END IF` line of the original `IF` statement.

**Format**
```
          IF <Boolean expression> THEN
          ~~BODY~~
          [ELSE IF <Boolean expression> THEN
          ~~BODY~~]*
          [ELSE
          ~~BODY~~]
          END IF
```

**Parameters**
N/A


476
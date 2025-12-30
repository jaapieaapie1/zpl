# DO Loops




ZBI Commands


**Example**
This is an example of how to use the IF statement command:

```
   10 IF A$="0" TyHEN
   20 PRINT "ZBI IS FUN"
   30 ELSE IF A$="1" THEN
   40 PRINT "ZBI IS EASY"
   50 ELSE IF TIME=1 THEN
   60 PRINT "It is one second past midnight"
   70 ELSE
   80 PRINT "X=0"
   90 END IF

```

Processing of the loop is controlled by a `<WHILE/UNTIL>` expression located on the `DO` or `LOOP` line.

Processing a `WHILE` statement is the same on either the `DO` or `LOOP` lines. The Boolean expression is
evaluated and if the statement is true, the `LOOP` continues at the line after the `DO` statement. Otherwise, the
line after the corresponding `LOOP` is the next line to be processed.

Processing an `UNTIL` statement is the same on either the `DO` or `LOOP` lines. The Boolean expression is
evaluated and if the statement is false, the `LOOP` continues at the line after the `DO` statement. Otherwise,
the line after the corresponding `LOOP` is the next to be processed.

If `<WHILE/UNTIL>` is on the `LOOP` line, the `BODY` of the loop is executed before the Boolean expression is
evaluated.

If neither the `DO` or `LOOP` line has a `<WHILE/UNTIL>` statement, the loop continues indefinitely.

Some notes about `DO-LOOP` s:

- can be nested


- cannot overlap


- have two formats


**Format**
```
   DO [<WHILE/UNTIL> <Boolean expression>]
   ~~BODY~~
   LOOP [<WHILE/UNTIL> <Boolean expression>]
```

**Example**
This is an example of how to use the `DO-LOOP` command with the conditional on the DO line:

```
   10 DO WHILE A$="70"
   20 INPUT A$
   30 LOOP

```

477


ZBI Commands


**Example**
This is an example of how to use the `DO UNTIL LOOP` command with conditional on the LOOP
line:

```
          10 DO
          20 INPUT A$
          30 LOOP UNTIL A$="EXIT"

```

**Comments**
This is a program command that is preceded by a line number.
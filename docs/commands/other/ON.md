# ON ERROR



The `ON ERROR` command can be used to prevent a program from halting in the event of an error. If an
error occurs in a previous line during program execution, the `ON ERROR` statement calls the `GOTO` or
`GOSUB` statement and allows the program to continue.

**Format**
```
          ON ERROR GOTO <A>
          ON ERROR GOSUB <A>
```

**Parameters**
`<A>` = the destination location in the program should an error be triggered on the previous line.

**Example**
This is an example of how to use the ON ERROR command:

```
          30 LET A = B/C
          40 ON ERROR GOTO 100
          ...
          100 PRINT "DIVIDE BY ZERO OCCURRED"
          110 LET A = 0
          120 GOTO 50
          ...

```

See TCP Server on page 504 or UDP Server on page 505.


**Comments**
If there is no error, this line is ignored.


This is a program command that is preceded by a line number.


536


ZBI Commands
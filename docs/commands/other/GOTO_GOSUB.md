# GOTO/GOSUB



`GOSUB` is followed by a line number. The program will attempt to process the line the `GOSUB` command
points to rather than the next line of the program. Upon executing the `GOSUB` statement, the interpreter
continues running at the line number specified following `GOSUB` . If the line number referenced does not
exist, an error will occur.

Before executing the next line, the `GOSUB` command stores the line number of the `GOSUB` line. When the
`RETURN` statement is called, the program moves back to the next line following the `GOSUB` .

Executing a `RETURN` statement without a corresponding `GOSUB` statement causes an error.

`GOSUB` statements can be nested.

`GOTO` works the same way as `GOSUB` except that no return address will be stored.

**Format**
```
          GOSUB <A>
          RETURN
          GOTO <A>
```

**Parameters**
`<A>` = the program location executed immediately after the GOTO or GOSUB.

**Example**
This is an example of how to use the `GOSUB` command:

```
          10 PRINT "Call Subroutine"
          20 GOSUB 1000
          30 PRINT "Returned from Subroutine"
          40 END
          1000 PRINT "In Subroutine"
          1010 RETURN

```

**Example**
This is an example of how to use the `GOTO` command:

```
          10 PRINT "Prepare to Jump!"
          20 GOTO 1000
          30 PRINT "Jump Missed..."
          1000 PRINT "Jump Successful"
          1010 END

```

**Comments**
These are program commands and must be preceded by line numbers.


480
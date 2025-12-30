# SUB




ZBI Commands


This command allows you to use names instead of actual line numbers as the target of `GOSUB` s and `GOTO` s.
`AUTONUM` can be used at the beginning of a file and there is no need to compute the line number where
the jump will go.


**Format**
```
   10 SUB <A>
```

**Parameters**
`<A>` = the integer variable to use as a target for the `GOTO` / `GOSUB`

**Example**
This is an example of how to use the `SUB` command:

```
   AUTONUM 1,1
   GOSUB INITCOMM
   DO
   GOSUB GETINPUT
   GOSUB PROCESSINPUT
   LOOP
   SUB INITCOMM
   OPEN #1:NAME "SER"
   RETURN
   SUB GETINPUT
   INPUT #1: A$
   RETURN
   SUB PROCESSINPUT
   PRINT A$
   RETURN

```

**Comments**
`<A>` is a numeric variable. If this variable is changed in the program, any `GOSUB/GOTO` to this
variable may fail.


481
# STORE




ZBI Commands


This command saves the program currently in memory as the specified file name. The format listed below
is used.


**Format**
```
   STORE <filename$>
```

**Parameters**
`<filename$>` = the name of the file to be stored. Drive location and file name must be in
quotation marks.

**Example**
This is an example of how to use the `STORE` command:

```
   STORE "E:PROGRAM1.BAS"

```

**Comments**
For a file name to be valid, it must conform to the 8.3 Rule: each file must have no more than eight
characters in the file name and have a three-character extension. Here the extension is always
`.BAS` (for example, `MAXIMUM8.BAS` ).

This is an interactive command that takes effect as soon as it is received by the printer.

The ZBI-Developer IDE will take care of this for you with the `SEND TO` option on your program.


510
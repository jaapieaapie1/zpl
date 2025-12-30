# PRINT




ZBI Commands


This command sends data to the printer to be printed.


**Format**
```
   PRINT [CHANNEL:] <expression> [,or; <expression>]* [;]
```

**Parameters**

`<CHANNEL>` = write data to this port

`<expression>` = the value to write

The expression can be either a string or a numeric expression.


    - Using a, to separate expressions adds a space between them.


    - Using a ; to separate expressions does not put a space between them.


    - Using a ; at the end of a line ends the print statement without adding a new line (CR/LF).


**Example**
This is an example of how to use the PRINT command:

```
   10 LET A$ = "This is an example"
   20 LET B$ = "of the PRINT Command."
   30 PRINT A$, B$ ! adds a space between expressions
   40 PRINT A$; B$ ! no space added
   RUN

```

The result is:


    - This is an example of the PRINT Command.


    - This is an exampleof the PRINT Command.


**Comments**
This can be an interactive command that takes effect as soon as it is received by the printer, or a
program command that is preceded by a line number.


496


ZBI Commands
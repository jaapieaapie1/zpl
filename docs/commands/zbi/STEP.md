# STEP




ZBI Commands


If a program was stopped by a `BREAK` command, `STEP` attempts to execute the program one line from
where it last ended. If the program has not been run or has been completed, this executes the lowest
numbered line.


**Format**
```
   STEP
```

**Parameters**
N/A

**Example**
This is an example of how to use the `STEP` command:

```
   10 PRINT "Hello World"
   20 Print "TWENTY"
   STEP
   Hello World

   STEP
   TWENTY

```

**Comments**
This is an interactive command that takes effect as soon as it is received by the printer.


460
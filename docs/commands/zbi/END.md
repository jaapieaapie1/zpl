# END




ZBI Commands


The `END` command terminates any program currently running. When the `END` command is received, the
interpreter returns to interpreting commands (>).


**Format**
```
   END
```

**Parameters**
N/A

**Example**
This is an example of how to use the `END` command:

```
   10 PRINT "THIS PROGRAM WILL TERMINATE"
   20 PRINT "WHEN THE END COMMAND IS RECEIVED"
   30 END
   40 PRINT "THIS SHOULD NOT PRINT"
   RUN
   THIS PROGRAM WILL TERMINATE
   WHEN THE END COMMAND IS RECEIVED

```

**Comments**
This is a program command and is preceded by a line number.


483


ZBI Commands
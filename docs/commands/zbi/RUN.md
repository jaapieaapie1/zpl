# RUN




ZBI Commands


This command executes the current program, starting with the lowest line number. The interpreter will
continue to execute the program lines in order unless a control statement directs the flow to a different
point. When a higher line number does not exist or the END command is processed, the `RUN` command will
stop.


**Format**
```
   RUN
```

**Parameters**
N/A

**Example**
This is an example of how to use the RUN command:

```
   10 PRINT "ZBI"
   20 PRINT "Programming"
   RUN
   ZBI
   Programming

   15 END
   RUN
   ZBI

```

**Comments**
Ports that are open when the application is activated will remain open after the application has
terminated. Variables also remain after the application has terminated.

To execute programs when the printer is powered on, use the `^JI` command in the Autoexec.zpl
file.


This is an interactive command that takes effect as soon as it is received by the printer.


457
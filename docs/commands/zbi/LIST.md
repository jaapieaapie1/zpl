# LIST




ZBI Commands


This command lists the program lines currently in memory.


**Format**
```
   LIST
   LIST <A>
   LIST <A>-<B>
```

**Parameters**


default = lists all lines in memory

`<A>` = line to start listing the program

`<B>` = line to stop listing the program. If not specified, only the line at `<A>` will print.

**Example**
This is an example of how to use the LIST command:

```
   1 REM MYPROGRAM COPYRIGHT ME Inc. 2008
   1 REM While debugging a port may be left open
   5 CLOSE ALL
   1 rem Open the ports this program will use
   10 OPEN #0: NAME: "SER" ! Restart the console
   20 PRINT #0: "Hello World"
   LIST
   1 REM Open the ports this program will use
   5 CLOSE ALL
   10 OPEN #0: NAME: "SER" ! Restart the console
   20 PRINT #0: "Hello World"

   LIST 1
   1 REM Open the ports this program will use

   LIST 5-10
   5 CLOSE ALL
   10 OPEN #0: NAME: "SER" ! Restart the console

```

**Comments**
The output of the `LIST` command may not match exactly what was entered. It is based on how the
program lines are stored in memory. Notice that the last comment line the REM is entered in lower
case characters. When it is listed, the REM is displayed in uppercase.


This is an interactive command that takes effect as soon as it is received by the printer.


451


ZBI Commands
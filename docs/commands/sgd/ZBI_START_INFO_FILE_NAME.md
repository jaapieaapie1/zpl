# zbi.start_info.file_name



This command prepares a program to run when the `zbi.start_info.execute` command is used. This
command does not run the program.


**Setvar**

To prepare a ZBI 2.0 program to be executed using the `zbi.start_info.execute` command:

```
       ! U1 setvar "zbi.start_info.file_name" "value"

```

**Values**


A file name or path of a basic program


**Getvar**

To return the file path and file name of a ZBI 2.0 program to run using the `zbi.start_info.execute`
command:

```
       ! U1 getvar "zbi.start_info.file_name"

```

**Default**

The last program run. If nothing has been run, `"*:\.BAZ"` .


**Example**

This `setvar` example shows the value set to `"E:PROGRAM1.BAS"` .

```
       ! U1 setvar "zbi.start_info.file_name" "E:PROGRAM1.BAS"

```

When the `setvar` value is set to `"E:PROGRAM1.BAS"`, the `getvar` result is `"E:PROGRAM1.BAS"` .


1066


SGD Printer Commands
# zbi.control.add_breakpoint



This command instructs the printer to set a ZBI program break point.


**Setvar**


To instruct the printer to set a ZBI program break point:

```
       ! U1 setvar "zbi.control.add_breakpoint" "value"

```

**Values**


Any line number of the program currently being debugged.


**Example**

This `setvar` example shows setting the breakpoint at line `"30"` .

```
       ! U1 setvar "zbi.control.add_breakpoint" "30"

```

1046


SGD Printer Commands
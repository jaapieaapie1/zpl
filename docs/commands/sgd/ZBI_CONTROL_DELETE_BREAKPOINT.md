# zbi.control.delete_breakpoint



This command deletes a breakpoint in the current ZBI 2.0 program.


**Setvar**


To instruct the printer to delete the breakpoint at the line indicated by the value parameter:

```
       ! U1 setvar "zbi.control.delete_breakpoint" "value"

```

**Values**

You can use the same value as `add_breakpoint` .


**Example**

This `setvar` example shows the breakpoint set to `"30"` .

```
       ! U1 setvar "zbi.control.delete_breakpoint" "30"

```

1049


SGD Printer Commands
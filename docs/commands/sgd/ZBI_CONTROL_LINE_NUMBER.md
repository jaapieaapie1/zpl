# zbi.control.line_number



This command gives you control and information about which line of a stopped ZBI 2.0 program is being
executed.


**Setvar**


To sets which line of the current ZBI 2.0 program should be executed:

```
       ! U1 setvar "zbi.control.line_number" "value"

```

**Values**


Any line number of the currently stopped ZBI program.


**Default**
```
          "0"

```

**Getvar**


To return the line number that is currently being executed in the ZBI 2.0 program:

```
       ! U1 getvar "zbi.control.line_number"

```

**Example**

This `setvar` example shows the value parameter set to `"30"` .

```
       ! U1 setvar "zbi.control.line_number" "30"

```

When the `setvar` value is set to `"30"`, the `getvar` result is `"30"` .


1050




SGD Printer Commands
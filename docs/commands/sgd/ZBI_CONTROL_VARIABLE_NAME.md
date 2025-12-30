# zbi.control.variable_name



This command sets the name of the variable that is to be read or modified through `variable_value` .


**Setvar**


To set the variable that is to show on the front panel:

```
       ! U1 setvar "zbi.control.variable_name" "value"

```

**Values**


Any ZBI variable in the program that is currently being debugged.


**Default**
```
          ""

```

**Getvar**


To retrieve the variable value that is to show on the front panel:

```
       ! U1 getvar "zbi.control.variable_name"

```

**Example**

This `setvar` example shows the value set to `"MYVAR$"` .

```
       ! U1 setvar "zbi.control.variable_name" "MYVAR$"

```

When the `setvar` value is set to `"MYVAR$"`, the `getvar` result is `"MYVAR$"` .


1055


SGD Printer Commands
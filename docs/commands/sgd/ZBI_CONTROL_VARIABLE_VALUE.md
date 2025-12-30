# zbi.control.variable_value



This command identifies the variable name.


**Setvar**


To set a value to the variable referenced by a variable_name:

```
       ! U1 setvar "zbi.control.variable_value" "value"

```

**Values**

A string or integer that is dependent on the variable type in `variable_name` .

**Default**


The current value of the variable referenced by variable_name.


**Getvar**


To retrieve the variable name that is loaded into the variable_name:

```
       ! U1 getvar "zbi.control.variable_value"

```

**Example**

This `setvar` example shows the value set to `"Hello World"` .

```
       ! U1 setvar "zbi.control.variable_value" "Hello World"

```

When the `setvar` value is set to `"Hello World"`, the `getvar` result is `"Hello World"` .


1056


SGD Printer Commands
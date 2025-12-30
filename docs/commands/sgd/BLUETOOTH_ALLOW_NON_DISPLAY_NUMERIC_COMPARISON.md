# bluetooth.allow_non_display_numeric_comparison



This command allows the user to enable or disable a numeric comparison connection for printers without a
display and also enables or disables displaying the pass key.


**NOTE:** This is an unsupported command for printers with display.


**Setvar**


To set the numeric comparison connection:

```
       ! U1 setvar "device.allow_non_display_numeric_comparison" "value"

```

**Values**

`"off"` Does not allow pass key connection for non display printers.

`"print"` Prints and accepts the pass key on non display printers (Man in the Middle
protection).

`"no print"` Accepts but does not print the pass key for non display printer (no Man in the
Middle protection).


**Default**
```
          "print"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "device.allow_non_display_numeric_comparison"

```

1091


SGD Network Commands
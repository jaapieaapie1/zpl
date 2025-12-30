# device.applicator.start_print_mode



Selects the applicator port START PRINT mode of operation.


**Setvar**


To select the start print mode of operation:

```
       ! U1 setvar "device.applicator.start_print_mode" "value"

```

**Values**

              - `"level"` the Start Print signal does not need to be de-asserted to print the next label. As long
as the Start Print signal is low and a label is formatted, a label prints.

              - `"pulse"` the Start Print signal must be de-asserted before it can be asserted for the next label

**Default**

```
          "pulse"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "device.applicator.start_print_mode"

```

**Result**

```
          "level"

```

**Example**

In the setvar example below, the `"level"` start print mode of operation is set.

```
       ! U1 setvar "device.applicator.start_print_mode" "level"

```

779


SGD Printer Commands
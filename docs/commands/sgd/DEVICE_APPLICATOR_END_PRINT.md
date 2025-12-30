# device.applicator.end_print



This command allows you to control an online verifier or applicator device.


**Setvar**

This command is similar to the `b` parameter for ^JJ on page 266.

To set the value for the applicator port mode:

```
       ! U1 setvar "device.applicator.end_print" "value"

```

**Values**

              - `"off"`

              - `"1"` End Print signal normally high, and low only when the printer is moving the label forward.

              - `"2"` End Print signal normally low, and high only when the printer is moving the label forward.

              - `"3"` End Print signal normally high, and low for 20 ms when a label has been printed and
positioned.

              - `"4"` End Print signal normally low, and high for 20 ms when a label has been printed and
positioned.


**Default**
```
          "off"

```

**Getvar**


To instruct the printer to respond with the currently set value:

```
       ! U1 getvar "device.applicator.end_print"

```

665


SGD Printer Commands
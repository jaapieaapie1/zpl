# power.power_on_mode



Indicates if the printer will power on automatically when power is applied, i.e., when the power supply is
plugged in.


**Getvar**


To return the current setting value:

```
       ! U1 getvar "power.power_on_mode"

```

**Values**

              - `"auto"` means the jumper is present on option card, which makes the printer power on
automatically when power is applied.

              - `"manual"` means jumper is not present on option card or the option card doesn't support autopower on, so the printer will power on only when the user presses the power button.

              - `"not available"` means this is not an option on the printer.

**Default**
```
          "on"

```

950




SGD Printer Commands
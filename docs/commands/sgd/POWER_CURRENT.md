# power.current



This command returns the battery pack instantaneous current value in mA for printers supporting Power
Precision Plus batteries.


Positive values indicate charging current, whereas negative values indicate discharging current.


**Getvar**


To return the battery pack instantaneous current value:

```
       ! U1 getvar "power.current"

```

**Result**

`"-32768"` to `"32767"` mA


**Example**

In the example below, the `getvar` returns the battery pack instantaneous current value.

```
       ! U1 getvar "power.current" "5643 mA"

```

954


SGD Printer Commands
# sensor.battery.in_volts



This command retrieves information on the battery current in volts.


**Getvar**


To return the current setting:

```
       ! U1 getvar "sensor.battery.in_volts"

```

**Result**

`"0.0"` to `"12.0"` volts


**Example**

In the `getvar` example below, the battery current volt reading of `"7.6"` is returned.

```
       ! U1 getvar "sensor.battery.in_volts" "7.6"

```

981


SGD Printer Commands
# sensor.head.temp



This command retrieves the current head temperature of the printer.


**Getvar**


To return the current average head temperature:

```
       ! U1 getvar "sensor.head.temp"

```

**Result**

`"-32768"` to `"32767"` Celsius


**Example**

In the `getvar` example below, the head temperature average of `"32"` is returned.

```
       ! U1 getvar "sensor.head.temp" "32"

```

999


SGD Printer Commands
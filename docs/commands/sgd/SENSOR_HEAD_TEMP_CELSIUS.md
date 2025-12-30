# sensor.head.temp_celsius



This command retrieves the current head temperature in Celsius.


**Getvar**


To return the current average head temperature:

```
       ! U1 getvar "sensor.head.temp_celsius"

```

**Result**

`"-32768"` to `"32767"` Celsius


**Example**

In the `getvar` example below, the head temperature average of `"0"` is returned.

```
       ! U1 getvar "sensor.head.temp_celsius" "0"

```

998


SGD Printer Commands
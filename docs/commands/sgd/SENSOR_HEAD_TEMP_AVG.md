# sensor.head.temp_avg



This command retrieves the current average head temperature in Celsius.


**Getvar**


To return the current average head temperature:

```
       ! U1 getvar "sensor.head.temp_avg"

```

**Result**

`"-32768"` to `"32767"` Celsius


**Example**

In the `getvar` example below, the head temperature average of `"32"` Celsius is returned.

```
       ! U1 getvar "sensor.head.temp_avg" "32"

```

997


SGD Printer Commands
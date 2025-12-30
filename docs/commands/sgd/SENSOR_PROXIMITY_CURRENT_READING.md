# sensor.proximity.current_reading



This command returns proximity information. The higher the number, the closer the item is.


**Getvar**


To return the proximity information from the sensor:

```
       ! U1 getvar "sensor.proximity.current_reading"

```

**Values**

              - `"0"` to `"4095"` A higher number indicates closer proximity.

              - `""` indicates that the sensor is not installed.


1007


SGD Printer Commands
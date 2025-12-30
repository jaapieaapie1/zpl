# sensor.object_temperature.current_reading



This command returns the temperature, in Celsius, from the sensor.


**Getvar**


To return the temperature from the object sensor:

```
       ! U1 getvar "sensor.object_temperature.current_reading"

```

**Values**

              - Object temperature, in degrees Celsius, from `"-20.0"` to `"200.0"` .

              - `""` indicates that the temperature sensor is not installed.


1001


SGD Printer Commands
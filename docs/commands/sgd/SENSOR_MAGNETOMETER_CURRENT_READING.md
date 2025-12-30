# sensor.magnetometer.current_reading



This command returns a magnetic induction reading in gauss.


**Getvar**


To return the magnetic induction reading from the sensor:

```
       ! U1 getvar "sensor.magnetometer.current_reading"

```

**Values**
The magnetic induction in gauss. If the sensor is not installed, the `getvar` returns `""` .


**Example**

In the following example, the `getvar` returns the magnetic induction reading from the sensor:

```
       ! U1 getvar "sensor.magnetometer.current_reading"

       "{"x":-12.3455,"y":1.2345,"z":0.1234}"
```

The `getvar` returns the magnetic field reading as a vector with x, y, and z components, each in the range
of +/-50 gauss.


1000




SGD Printer Commands
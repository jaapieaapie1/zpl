# device.sensor_select



Determines which media sensor will be used.

This command is similar to the `^JS` ZPL command.


**Setvar**


To determine which media sensor will be used:

```
       ! U1 setvar "device.sensor_select" "value"

```

**Values**

              - `"reflective"`

              - `"transmissive"`


**Getvar**


To retrieve which media sensor is used:

```
       ! U1 getvar "device.sensor_select"

```

748


SGD Printer Commands
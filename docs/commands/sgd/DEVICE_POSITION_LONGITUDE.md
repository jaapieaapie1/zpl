# device.position.longitude



This printer setting retrieves/sets the geographic longitudinal position.


**Setvar**


To set the longitudinal position of the printer:

```
       ! U1 setvar "device.position.longitude" "value"

```

**Values**


              - The value is in decimal degrees from 0.0 to +/-180.0.


              - The value is saved as a double precision floating point number.


**Default**
```
          "0.0"

```

**Getvar**


To retrieve the longitudinal position of the printer:

```
       ! U1 getvar "device.position.longitude"

```

**Values**


The value is returned with 6 decimal places. A value of 0.000001 degree is on the order of no
more than 0.1 meter of distance on the earth's surface. (The correspondence between degrees and
length on the earth's surface varies from approximately 0.1 meter at the equator to 0.0 at the poles.)


**Example**

```
       ! U1 setvar "device.position.longitude" "25.123456"

```

731


SGD Printer Commands
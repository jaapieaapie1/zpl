# device.position.latitude



This printer setting retrieves/sets the geographic latitudinal position.


**Setvar**


To set the latitude position of the printer:

```
       ! U1 setvar "device.position.latitude" "value"

```

**Values**


The value is in decimal degrees from 0.0 to +/-90.0.


**Default**
```
          "0.0"

```

**Getvar**


To retrieve the latitude position of the printer:

```
       ! U1 getvar "device.position.latitude"

```

**Values**

The value is returned with 6 decimal places. A value of `0.000001` degree is on the order of 0.1
meter of distance on the earth's surface. (The correspondence between degrees and length on the
earth's surface varies because the earth is an irregular ellipsoid.)


**Example**

```
       ! U1 setvar "device.position.latitude" "6.123456"

```

730




SGD Printer Commands
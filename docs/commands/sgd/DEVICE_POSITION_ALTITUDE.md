# device.position.altitude



This printer setting retrieves the altitude above sea level.


The value is in meters above sea level. A positive number indicates a position above sea level. A negative
number indicates a position below sea level. The position of sea level depends upon the system used to
provide a nominal sea level reference position. This is often the World Geodetic System WGS 84 standard
but depends upon the location provider.


**Setvar**


To set the altitude of the printer above sea level:

```
       ! U1 setvar "device.position.altitude" "value"

```

**Values**


A decimal number with 6 decimal places, e.g. 305.100000. The value is saved as a double
precision floating point number.

              - Minimum: `"-10000"`

              - Maximum: `"406700000"`


**Getvar**


To retrieve the altitude above sea level:

```
       ! U1 getvar "device.position.altitude"

```

**Example**

```
       ! U1 setvar "device.position.altitude" "305.1"

```

729


SGD Printer Commands
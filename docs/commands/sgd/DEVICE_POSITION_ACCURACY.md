# device.position.accuracy



This printer setting retrieves/sets the accuracy of the geographic position values.


The units of the value depend upon the location provider that was used to determine the geographic
coordinates. Usually, this is specified as a radius, in meters, of confidence around the location coordinates.
Often, the radius represents a radius of 68% confidence that the true location lies within the circle,
representing one standard deviation.


**NOTE:** These settings hold the value to which they are set, within the range restrictions. The
printer does not perform any calculations, nor associate any meaning such as “meters” or “feet”
to the values. The values can be determined by a number of methods, including an Android® or
iOS® application communicating with the printer using the smart phone’s geolocation device.


**Setvar**


To set the accuracy of the geographic position values:

```
       ! U1 setvar "device.position.accuracy" "value"

```

**Values**


A decimal number with 6 decimal places, e.g. 25.370000. The value is saved as a double precision
floating point number.

              - Minimum: `"0"`

              - Maximum: `"406700000"`


**Getvar**


To retrieve the accuracy of the geographic position values:

```
       ! U1 getvar "device.position.accuracy"

```

**Example**

```
       ! U1 setvar "device.position.accuracy" "25.37"

```

728


SGD Printer Commands
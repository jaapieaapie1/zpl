# odometer.net_media_length



This command displays the total amount of media used by the printer. The printer tracks all forward motion,
subtracting all backward motion, so the odometer always indicates the total amount of media consumed by
the printer. The media is measured in inches and centimeters.


**Getvar**


To return the current length, in centimeters, of media used by the printer:

```
       ! U1 getvar "odometer.net_media_length"

```

**Result**


This example shows that the printer used 9,492 inches, or 24,111 centimeters, of media:

```
          "9492 INCHES, 24111 CENTIMETERS"

```

924


SGD Printer Commands
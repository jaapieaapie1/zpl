# odometer.net_ribbon_length



This command displays the total amount of ribbon used by the printer. All forward and backward motion is
tracked to determine the amount of ribbon used. The ribbon length is measured in inches and centimeters.


**Getvar**


To return the current length, in centimeters, of ribbon used by the printer:

```
       ! U1 getvar "odometer.net_ribbon_length"

```

**Result**


This example shows that the printer used 244 inches, or 621 centimeters, of ribbon:

```
          "244 INCHES, 621 CENTIMETERS"

```

925


SGD Printer Commands
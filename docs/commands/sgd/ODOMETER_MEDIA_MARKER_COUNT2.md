# odometer.media_marker_count2



This printer setting refers to the value of the second (count2) user resettable counter. The user resettable
counters track how much media has passed through the printer in both inches or centimeters.


**Setvar**


To reset the second user resettable counter:

```
       ! U1 setvar "odometer.media_marker_count2" "value"

```

**Values**

`"0"` resets the counter

**Default**


Must be an accepted value or it is ignored.


**Getvar**


To return the current value of the second (count2) user resettable counter in both inches and centimeters:

```
       ! U1 getvar "odometer.media_marker_count2"

```

**Example**


This example shows how to get the second user resettable counter, how to reset it, and how to confirm the
settings have changed:


To see the current settings, type:

```
       ! U1 getvar "odometer.media_marker_count2"

```

Something similar to this is shown:

```
       "8516 INCHES, 21632 CENTIMETERS"

```

To reset the these values to 0, type:

```
       ! U1 setvar "odometer.media_marker_count2" "0"

```

To confirm these settings were reset, type:

```
       ! U1 getvar "odometer.media_marker_count2"

```

If the resetting was successful, this is shown:

```
       "0 INCHES, 0 CENTIMETERS"

```

923


SGD Printer Commands
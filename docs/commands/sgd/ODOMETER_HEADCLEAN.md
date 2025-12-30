# odometer.headclean



This printer setting refers to the head clean odometer count. This counter tracks how many inches and
centimeters have passed through the printer since the head was last cleaned.


**Setvar**


To reset the head clean counter:

```
       ! U1 setvar "odometer.headclean" "value"

```

**Values**

`"0"` resets the head clean counter

**Default**


Must be an accepted value or it is ignored


**Getvar**


To retrieve the values for the head clean counter:

```
       ! U1 getvar "odometer.headclean"

```

**Example**


This example shows how to get the odometer head clean, how to reset it, and how to confirm the settings
changed.


To see the current settings, type:

```
       ! U1 getvar "odometer.headclean"

```

Something similar to this is shown:

```
       "1489 INCHES, 3784 CENTIMETERS"

```

To reset the these values to 0, type:

```
       ! U1 setvar "odometer.headclean" "0"

```

To confirm this settings were reset, type:

```
       ! U1 getvar "odometer.headclean"

```

If the resetting was successful, this is shown:

```
       "0 INCHES, 0 CENTIMETERS"

```

918


SGD Printer Commands
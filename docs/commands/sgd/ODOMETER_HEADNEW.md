# odometer.headnew



This printer setting refers to the head replaced odometer count. This counter tracks how many inches and
centimeter passed through the printer since the head was last replaced.


**Setvar**


To instruct the printer to reset the head new counter:

```
       ! U1 setvar "odometer.headnew" "value"

```

**Values**

`"0"` resets the head new counter

**Default**


Must be an accepted value or it is ignored


**Getvar**


To instruct the printer to retrieve the values for the head new counter:

```
       ! U1 getvar "odometer.headnew"

```

**Example**


This example shows how to get the odometer head new, how to reset it, and how to confirm the settings
changed:


To see the current settings, type:

```
       ! U1 getvar "odometer.headnew"

```

Something similar to this is shown:

```
       "1489 INCHES, 3784 CENTIMETERS"

```

To reset the these values to 0, type:

```
       ! U1 setvar "odometer.headnew" "0"

```

To confirm this settings were reset, type:

```
       ! U1 getvar "odometer.headnew"

```

If the resetting was successful, this is shown:

```
       "0 INCHES, 0 CENTIMETERS"

```

919


SGD Printer Commands
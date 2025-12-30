# wlan.roam.interval



This printer setting refers to specifying the wireless roam interval.


**Setvar**


This command instructs the printer to set the wireless roam interval.

```
       ! U1 setvar "wlan.roam.interval"

```

**Values**
Decimal values between `"5"` and `"255"` inclusive

**Default**
```
          "20"

```

**Getvar**


This command instructs the printer to respond with the specified roam interval.

```
       ! U1 getvar "wlan.roam.interval"

```

**Example**

This `setvar` example shows the value set to `"20"` .

```
       ! U1 setvar "wlan.roam.interval" "20"

```

When the `setvar` value is set to `"20"`, the `getvar` result is `"20"` .


1470




SGD Network Commands
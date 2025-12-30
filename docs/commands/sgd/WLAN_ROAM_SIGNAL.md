# wlan.roam.signal



This printer setting refers to specifying the wireless roam signal.


**Setvar**


To set the wireless roam signal:

```
       ! U1 setvar "wlan.roam.signal" "value"

```

**Values**


Decimal values between 1 and 75 inclusive.


**Default**
```
          "50"

```

**Getvar**


To respond with the specified wireless roam signal:

```
       ! U1 getvar "wlan.roam.signal"

```

**Example**

This `setvar` example shows the value set to `"50"` .

```
       ! U1 setvar "wlan.roam.signal" "50"

```

When the `setvar` value is set to `"50"`, the `getvar` result is `"50"` .


1476


SGD Network Commands
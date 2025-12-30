# wlan.leap_username



This printer setting refers to the LEAP user name. The user name must correspond to a user profile
established on the RADIUS/AAA server in use.


**NOTE:** This parameter is not supported on units with a Frequency Hopping Spread Spectrum
(FHSS) radio.


**Setvar**


To change the LEAP user name:

```
       ! U1 setvar "wlan.leap_username" "values"

```

**Values**


0 to 32 alphanumeric ASCII characters.


**Default**
```
          "user"

```

**Getvar**


To respond with the LEAP user name:

```
       ! U1 getvar "wlan.leap_username"

```

**Example**

This setvar example shows the value set to `"user"` .

```
       ! U1 setvar "wlan.leap_username" "user"

```

When the `setvar` value is set to `"user"`, the `getvar` result is `"user"` .


1458


SGD Network Commands
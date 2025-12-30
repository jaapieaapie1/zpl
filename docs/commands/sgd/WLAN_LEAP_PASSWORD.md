# wlan.leap_password



This printer setting refers to the LEAP password. The password must correspond to a user profile
established on the RADIUS/AAA server in use.


**NOTE:** This parameter is not supported on units with a Frequency Hopping Spread Spectrum
(FHSS) radio.


**Setvar**


This command instructs the printer to change the LEAP password.

```
       ! U1 setvar "wlan.leap_password" "values"

```

**Values**
0 to 32 ASCII characters

**Default**
```
          "password"

```

**Getvar**


This command instructs the printer to respond with the LEAP password.

```
       ! U1 getvar "wlan.leap_password"

```

For protection, a single `"*"` prints.


**Example**

This setvar example shows the value set to `"password"` .

```
       ! U1 setvar "wlan.leap_password" "password"

```

When the `setvar` value is set to `"password"`, the `getvar` result is `"*"` .


1457


SGD Network Commands
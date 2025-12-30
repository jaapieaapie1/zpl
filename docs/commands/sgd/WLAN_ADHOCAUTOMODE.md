# wlan.adhocautomode



This printer setting refers to enabling or disabling the adhoc auto mode.


**Setvar**


To instruct the printer to set the adhoc auto mode:

```
       ! U1 setvar "wlan.adhocautomode" "value"

```

**Values**

              - `"on"` adhoc auto mode enabled

              - `"off"` adhoc auto mode disabled

**Default**
```
          "off"

```

**Getvar**


To instruct the printer to respond with the adhoc auto mode status:

```
       ! U1 getvar "wlan.adhocautomode"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "wlan.adhocautomode" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1386


SGD Network Commands
# wlan.keep_alive.enable



This setting controls the printers ability to send a LSAP (link service access point) packet to the access
point on an user controllable interval. This feature is included to accommodate access points that require a
regular confirmation that wireless clients are still active.


**Setvar**


To enable or disable the keep alive printer setting:

```
       ! U1 setvar "wlan.keep_alive.enable" "value"

```

**Values**

`"on"` turns on keep_alive

`"off"` turns off keep_alive

**Default**
```
          "on"

```

**Getvar**

To return with the current value of the `wlan.keep_alive.enable` setting:

```
       ! U1 getvar "wlan.keep_alive.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "wlan.keep_alive.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"`


1449


SGD Network Commands
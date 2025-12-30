# wlan.ip.timeout.enable



Use this command to enable the connection timeout on the wireless print server. For this to take effect, the
print server must be reset.


**Setvar**


To enable or disable the timeout checking on the wireless print server:

```
       ! U1 setvar "wlan.ip.timeout.enable" "value"

```

**Values**
`"off"` turns off the connection checking

`"on"` turns on the connection checking

**Default**
```
          "on"

```

**Getvar**


To return whether the timeout checking is enabled on the wireless print server:

```
       ! U1 getvar "wlan.ip.timeout.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "wlan.ip.timeout.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1433


SGD Network Commands
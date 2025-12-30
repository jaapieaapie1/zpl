# wlan.ip.dhcp.session_interval



This command retrieves how long it will take for a DHCP session to time out before a new DHCP session
begins on the wireless print server.


**Setvar**


To set the DHCP session time out:

```
       ! U1 setvar "wlan.ip.dhcp.session_interval" "value"

```

**Values**

`"0"` through `"60"`

**Default**
```
          "10"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.ip.dhcp.session_interval"

```

**Example**

This `setvar` example shows the value set to `"10"` .

```
       ! U1 setvar "wlan.ip.dhcp.session_interval" "10"

```

When the `setvar` value is set to `"10"`, the `getvar` result is `"10"` .


1422


SGD Network Commands
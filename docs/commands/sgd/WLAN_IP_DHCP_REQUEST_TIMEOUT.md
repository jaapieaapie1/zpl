# wlan.ip.dhcp.request_timeout



This command sets the maximum time (in seconds) to wait for a response to a DHCP discovery request on
the wireless print server.


**Setvar**


To set the maximum time (in seconds) to wait for a response to a DHCP discovery request on the wireless
print server:

```
       ! U1 setvar "wlan.ip.dhcp.request_timeout" "value"

```

**Values**

`"2"` through `"30"`

**Default**
```
          "2"

```

**Getvar**


To retrieve the maximum time (in seconds) to wait for a response to a DHCP discovery request on the
wireless print server:

```
       ! U1 getvar "wlan.ip.dhcp.request_timeout"

```

**Example**

This `setvar` example shows the value set to `"2"` .

```
       ! U1 setvar "wlan.ip.dhcp.request_timeout" "2"

```

When the `setvar` value is set to `"2"`, the `getvar` result is `"2"` .


1420




SGD Network Commands
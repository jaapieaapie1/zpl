# wlan.ip.dhcp.requests_per_session



This command retrieves the maximum amount of DHCP discover requests for a single DHCP session on
the wireless print server.


**Setvar**


To set the maximum amount of DHCP discover requests for a single DHCP session on the wireless print
server:

```
       ! U1 setvar "wlan.ip.dhcp.requests_per_session" "value"

```

**Values**

`"1"` through `"10"`

**Default**
```
          "2"

```

**Getvar**


To retrieve the currently set maximum amount of DHCP discover requests for a single DHCP session on
the wireless print server:

```
       ! U1 getvar "wlan.ip.dhcp.requests_per_session"

```

**Example**

This `setvar` example shows the value set to `"2"` .

```
       ! U1 setvar "wlan.ip.dhcp.requests_per_session" "2"

```

When the `setvar` value is set to `"2"`, the `getvar` result is `"2"` .


1421


SGD Network Commands
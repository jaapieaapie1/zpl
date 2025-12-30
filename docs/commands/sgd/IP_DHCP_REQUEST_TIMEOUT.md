# ip.dhcp.request_timeout



This command retrieves the maximum amount of time (in seconds) for a DHCP discovery requests on the
wireless print server.


**Setvar**


To set the amount of time (in seconds) to wait before timing out a DHCP discovery request:

```
       ! U1 setvar "ip.dhcp.request_timeout" "value"

```

**Values**

`"2"` through `"30"`

**Default**
```
          "2"

```

**Getvar**


To retrieve the currently set the amount of time (in seconds) to wait before timing out a DHCP discovery
request:

```
       ! U1 getvar "ip.dhcp.request_timeout"

```

**Example**

This `setvar` example shows the value set to `"2"` .

```
       !
       U1 setvar "ip.dhcp.request_timeout" "2"

```

When the `setvar` value is set to `"2"`, the `getvar` result is `"2"` .


1239


SGD Network Commands
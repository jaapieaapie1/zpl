# ip.dhcp.session_interval



This command configures the time interval (in seconds) before a new DHCP session is started on the
wireless print server.


**Setvar**


To set the DHCP session time out value (in seconds):

```
       ! U1 setvar "ip.dhcp.session_interval" "value"

```

**Values**

`"0"` through `"60"`

**Default**
```
          "10"

```

**Getvar**


To retrieve the current DHCP session time out value (in seconds):

```
       ! U1 getvar "ip.dhcp.session_interval"

```

**Example**

This `setvar` example shows the value set to `"10"` .

```
       ! U1 setvar "ip.dhcp.session_interval" "10"

```

When the `setvar` value is set to `"10"`, the `getvar` result is `"10"` .


1241


SGD Network Commands
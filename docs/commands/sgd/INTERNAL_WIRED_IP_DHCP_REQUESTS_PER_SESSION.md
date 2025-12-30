# internal_wired.ip.dhcp.requests_per_session



This command retrieves the maximum amount of DHCP discover requests for a single DHCP session on
the internal wired print server.


**Setvar**


To instruct the printer to set the maximum amount of DHCP discover requests for a single DHCP session
on the internal wired print server:

```
       ! U1 setvar "internal_wired.ip.dhcp.requests_per_session" "value"

```

**Values**


1-10


**Default**
```
          "2"

```

**Getvar**


To retrieve the currently set maximum amount of DHCP discover requests for a single DHCP session on
the internal wired print server:

```
       ! U1 getvar "internal_wired.ip.dhcp.requests_per_session"

```

**Example**

This `setvar` example shows the value set to `"2"` .

```
       ! U1 setvar "internal_wired.ip.dhcp.requests_per_session" "2"

```

When the `setvar` value is set to `"2"`, the `getvar` result is `"2"` .


1186


SGD Network Commands
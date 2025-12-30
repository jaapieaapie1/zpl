# wlan.ip.port



This printer setting refers to the wireless print server’s port number that the TCP print service is listening
on. Normal TCP communications from the host should be directed to this port.


**Setvar**


To set the wireless print server’s TCP/UDP port number:

```
       ! U1 setvar "wlan.ip.port" "value"

```

**Values**

`"1"` through `"65535"` (excluding any ports currently used by other services, such as 21, 23, 80,
and 515).


**Default**
```
          "9100"

```

**Getvar**


To respond with the wireless printer server’s TCP/UDP port number:

```
       ! U1 getvar "wlan.ip.port"

```

**Example**

This `setvar` example shows the value set to `"9100"` .

```
       ! U1 setvar "wlan.ip.port" "9100"

```

When the `setvar` value is set to `"9100"`, the `getvar` result is `"9100"` .


1429


SGD Network Commands
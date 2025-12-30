# ip.port




SGD Network Commands


This printer setting refers to the port number that the TCP print service is listening on. Normal TCP
communications from the host should be directed to this port.


**Setvar**


To set the TCP/UDP port number:

```
! U1 setvar "ip.port" "value"

```

**Values**

`"1"` through `"65535"` (excluding any ports currently used by other services, such as 21, 23, 80,
and 515).


**Default**
```
   "9100"

```

**Getvar**


To respond with the TCP/UDP port number:

```
! U1 getvar "ip.port"

```

**Example**

This `setvar` example shows the value set to `"9100"` .

```
! U1 setvar "ip.port" "9100"

```

When the `setvar` value is set to `"9100"`, the `getvar` result is `"9100"` .


1304


SGD Network Commands
# ip.dhcp.cid_suffix



This printer setting defines the unique suffix to be used as the client identifier (DHCP option 61) if DHCP is
enabled and `"ip.dhcp.cid_type"` is set to `"0"` or `"2"` .


**Setvar**


To instruct the printer to change the CID value:

```
       ! U1 setvar "ip.dhcp.cid_suffix" "value"

```

**Values**


The maximum length of a value allowed is 60 ASCII characters when the CID type is ASCII, or 120
hexadecimal values when the CID type is hexadecimal.


**Default**
```
          ""

```

**Getvar**


To instruct the printer to respond with the client identifier suffix:

```
       ! U1 getvar "ip.dhcp.cid_suffix"

```

**Example**

This `setvar` example shows the value set to `"printer"` .

```
       ! U1 setvar "ip.dhcp.cid_suffix" "printer"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"printer"` .


1226


SGD Network Commands
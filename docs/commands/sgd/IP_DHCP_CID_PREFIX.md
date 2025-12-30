# ip.dhcp.cid_prefix



This printer setting defines the prefix to be prepended to the DHCP client identifier (option 61) when DHCP
is enabled and `"ip.dhcp.cid_type"` is set to `"0"` or `"2".`


**Setvar**


To instruct the printer to change the CID prefix:

```
       ! U1 setvar "ip.dhcp.cid_prefix" "value"

```

**Values**


Any text string up to 10 characters if the CID type is ASCII, or 20 characters if the CID type is
hexadecimal.


**Default**
```
          ""

```

**Getvar**


To instruct the printer to respond with the client identifier prefix:

```
       ! U1 getvar "ip.dhcp.cid_prefix"

```

**Example**

This `setvar` example sets the value to `"ZEB"` .

```
       ! U1 setvar "ip.dhcp.cid_prefix" "ZEB"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"ZEB"` .

The next time the printer sends a DHCP request, if `ip.dhcp.cid_type` is `"0"`, the client identifier sent
will be prefixed with the string `"ZEB"` . For example, if `ip.dhcp.cid_value` is `"PRT001"`, the actual
client identifier sent will be `"ZEBPRT001"`

.


1225


SGD Network Commands
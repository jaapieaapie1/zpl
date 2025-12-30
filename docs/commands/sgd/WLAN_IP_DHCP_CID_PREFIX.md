# wlan.ip.dhcp.cid_prefix



This printer setting defines the prefix to be prepended to the DHCP client identifier (option 61) when DHCP
is enabled on the wireless print server and `"wlan.ip.dhcp.cid_type"` is set to `"0"` or `"2"` .


**Setvar**


To instruct the printer to change the CID prefix of the wireless print server:

```
       ! U1 setvar "wlan.ip.dhcp.cid_prefix" "value"

```

**Values**


Any text string up to 10 characters if the CID type is ASCII, or 20 characters if the CID type is
hexadecimal.


**Default**
```
          ""

```

**Getvar**


To instruct the printer to respond with the client identifier prefix of the wireless print server:

```
       ! U1 getvar "wlan.ip.dhcp.cid_prefix"

```

**Example**

This `setvar` example shows the value set to `"PRT001"` .

```
       ! U1 setvar "wlan.ip.dhcp.cid_prefix" "PRT001"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"PRT001"` .


1410




SGD Network Commands
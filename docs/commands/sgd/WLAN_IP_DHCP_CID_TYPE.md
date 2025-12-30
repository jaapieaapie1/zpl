# wlan.ip.dhcp.cid_type



This printer setting defines the type of client identifier (DHCP option 61) that will be sent if DHCP is enabled
on the wireless print server. A value of `"1"` means the type of Ethernet and the printer’s MAC address will
be used. A value of `"0"` or `"2"` means the client identifier sent will be `"wlan.ip.dhcp.cid_prefix"`
concatenated with `"wlan.ip.dhcp.cid_suffix"` .


**Setvar**


To instruct the printer to enable "synthetic" client identifier for the wireless print server:

```
       ! U1 setvar "wlan.ip.dhcp.cid_type" "value"

```

**Values**

              - `"0"` means ASCII string

              - `"1"` means MAC address of the wireless radio card

              - `"2"` means HEX value

**Default**
```
          "1"

```

**Getvar**


To instruct the printer to respond with the client identifier type for the wireless print server:

```
       ! U1 getvar "wlan.ip.dhcp.cid_type"

```

**Example**

This `setvar` example shows the value set to `"1"` .

```
       ! U1 setvar "wlan.ip.dhcp.cid_type" "1"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"1"` .


1412


SGD Network Commands
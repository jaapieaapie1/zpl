# ip.dhcp.cid_type



This printer setting defines the type of client identifier (DHCP option 61) that will be sent if DHCP is
enabled. A value of `"1"` means the type of "Ethernet" and the printer’s MAC address will be used. A
value of `"0"` or `"2"` means the client identifier sent will be `"ip.dhcp.cid_prefix"` concatenated with
`"ip.dhcp.cid_suffix"` .


**Setvar**


To instruct the printer to set the Client Identifier type:

```
       ! U1 setvar "ip.dhcp.cid_type" "value"

```

**Values**

              - `"0"` ASCII string

              - `"1"` MAC address of the wireless radio card

              - `"2"` HEX value

**Default**
```
          "1"

```

**Getvar**


To instruct the printer to respond with the client identifier type:

```
       ! U1 getvar "ip.dhcp.cid_type"

```

**Example**

This `setvar` example shows the value set to `"1"` .

```
       ! U1 setvar "ip.dhcp.cid_type" "1"

```

When the `setvar` value is set to `"1"`, the `getvar` result is `"1"` .


1227


SGD Network Commands
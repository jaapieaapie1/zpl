# internal_wired.ip.dhcp.cid_type



This printer setting defines the type of client identifier (DHCP option 61) that will be sent
if DHCP is enabled on the internal wired print server. A value of `"1"` means the type of
"Ethernet" and the printer’s MAC address will be used.A value of `"0"` or `"2"` means the client
identifier sent will be `"internal_wired.ip.dhcp.cid_prefix"` concatenated with
`"internal_wired.ip.dhcp.cid_suffix"` .


**Setvar**


To instruct the printer to enable "synthetic" Client Identifier for the internal wired print server:

```
       ! U1 setvar "internal_wired.ip.dhcp.cid_type" "value"

```

**Values**

              - `"0"` ASCII string

              - `"1"` wired print server’s MAC address

              - `"2"` HEX value


**Getvar**


To instruct the printer to respond with the client identifier type for the internal wired print server:

```
       ! U1 getvar "internal_wired.ip.dhcp.cid_type"

```

**Example**

This `setvar` example shows the value set to `"1"` .

```
       ! U1 setvar "internal_wired.ip.dhcp.cid_type" "1"

```

When the `setvar` value is set to `"1"`, the `getvar` result is `"1"` .


1178


SGD Network Commands
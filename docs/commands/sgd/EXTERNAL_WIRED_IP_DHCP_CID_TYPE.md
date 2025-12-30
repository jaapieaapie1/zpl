# external_wired.ip.dhcp.cid_type



This printer setting defines the type of client identifier (DHCP option 61) that will be sent
if DHCP is enabled on the external wired print server. A value of `"1"` means the type of
"Ethernet" and the printer’s MAC address will be used. A value of `"0"` or `"2"` means the client
identifier sent will be `"external_wired.ip.dhcp.cid_prefix"` concatenated with
`"external_wired.ip.dhcp.cid_suffix"` .


**Setvar**


To instruct the printer to enable "synthetic" Client Identifier for the external wired print server:

```
       ! U1 setvar "external_wired.ip.dhcp.cid_type" "value"

```

**Values**

              - `"0"` uses an ASCII string

              - `"1"` uses MAC address of wired print server

              - `"2"` uses HEX value

**Default**
```
          "1"

```

**Getvar**


To instruct the printer to respond with the client identifier type for the external wired print server:

```
       ! U1 getvar "external_wired.ip.dhcp.cid_type"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**Example**

This `setvar` example shows the value set to `"1"` .

```
       ! U1 setvar "external_wired.ip.dhcp.cid_type" "1"

```

When the `setvar` value is set to `"1"`, the `getvar` result is `"1"` .


1123


SGD Network Commands
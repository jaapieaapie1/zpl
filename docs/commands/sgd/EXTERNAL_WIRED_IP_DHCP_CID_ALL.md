# external_wired.ip.dhcp.cid_all



This printer setting defines the entire client identifier (DHCP option 61) if DHCP is enabled on the external
print server and `"external_wired.ip.dhcp.cid_type"` is set to `"0"`, or `"2"` . The MAC address is
used if the type is set to `"1"` .


**Setvar**


To instruct the printer to change the client identifier prefix and suffix of the external wired print server:

```
       ! U1 setvar "external_wired.ip.dhcp.cid_all" "value"

```

**Values**


A maximum length of 60 characters if the CID type is ASCII, or 120 characters if the CID type is
hexadecimal.


**Default**
```
          ""

```

**Getvar**


To instruct the printer to respond with the client identifier prefix and suffix of the external wired print server:

```
       ! U1 getvar "external_wired.ip.dhcp.cid_all"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**Example**

This `setvar` example shows the value set to `"printer"` .

```
       ! U1 setvar "external_wired.ip.dhcp.cid_all" "printer"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"printer"` .


1119


SGD Network Commands
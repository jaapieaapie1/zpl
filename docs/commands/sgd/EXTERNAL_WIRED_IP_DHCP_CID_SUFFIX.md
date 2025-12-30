# external_wired.ip.dhcp.cid_suffix



This printer setting defines the unique suffix to be used as the client identifier (DHCP option 61) if DHCP is
enabled repeated on the external wired print server and `external_wired.ip.dhcp.cid_type on`
`page 1077` is set to `"0"` or `"2"`, not `"1"` .


**Setvar**


To instruct the printer to change the client identifier suffix value:

```
       ! U1 setvar "external_wired.ip.dhcp.cid_suffix" "value"

```

**Values**


The maximum length of a value allowed is 60 ASCII characters when the CID type is ASCII, or 120
hexadecimal values when the CID type is hexadecimal.


**Default**
```
          ""

```

**Getvar**


To instruct the printer to respond with the client identifier suffix on the external wired print server:

```
       ! U1 getvar "external_wired.ip.dhcp.cid_suffix"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**Example**

This `setvar` example shows setting the suffix to `"printer"` .

```
       ! U1 setvar "external_wired.ip.dhcp.cid_suffix" "printer"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"printer"` .


1122


SGD Network Commands
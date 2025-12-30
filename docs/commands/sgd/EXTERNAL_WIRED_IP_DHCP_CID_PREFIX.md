# external_wired.ip.dhcp.cid_prefix



This printer setting defines the prefix to be prepended to the DHCP client identifier (option 61) when DHCP
is enabled on the external wired print server and `"external_wired.ip.dhcp.cid_type"` is set to
`"0"` or `"2"` .


**Setvar**


To instructs the printer to change the CID prefix of the external wired print server:

```
       ! U1 setvar "external_wired.ip.dhcp.cid_prefix" "value"

```

**Values**


Any text string up to 10 characters if the CID type is ASCII, or 20 characters if the CID type is
hexadecimal.


**Default**
```
          ""

```

**Getvar**


To instruct the printer to respond with the client identifier prefix of the external wired print server:

```
       ! U1 getvar "external_wired.ip.dhcp.cid_prefix"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**Example**

This `setvar` example shows the value set to `"PRT001"` .

```
       ! U1 setvar "external_wired.ip.dhcp.cid_prefix" "PRT001"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"PRT001"` .


1121


SGD Network Commands
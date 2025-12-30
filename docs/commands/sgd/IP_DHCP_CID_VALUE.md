# ip.dhcp.cid_value



This parameter defines the unique value to be used as the client identifier (option 61) if DHCP is enabled
and `"ip.dhcp.cid_type"` is `"1"` .


**NOTE:** This parameter is only applicable if `ip.dhcp.cid_enable` is set to `"on"` .


**Setvar**


To instruct the printer to change the CID value:

```
       ! U1 setvar "ip.dhcp.cid_value" "value"

```

**Values**


Any text string up to 20 characters in length.


**Default**

If `ip.dhcp.cid_type` is set to:

              - `"0"` the default is the printer’s friendly name.

              - `"1"` the default is the MAC address of the printer.


**Getvar**


To instruct the printer to respond with the client identifier value:

```
       ! U1 getvar "ip.dhcp.cid_value"

```

**Example**

This `setvar` example changes the cid value to `"PRT001"` .

```
       ! U1 setvar "ip.dhcp.cid_value" "PRT001"

```

The next time the printer sends a DHCP request, if `ip.dhcp.cid_type` is `"0"`, the client identifier sent
will be `ip.dhcp.cid_prefix` plus `"PRT001".` For example, if `ip.dhcp.cid_prefix` is `"ZEB"`, the
actual client identifier sent will be `"ZEBPRT001"` .


1228


SGD Network Commands
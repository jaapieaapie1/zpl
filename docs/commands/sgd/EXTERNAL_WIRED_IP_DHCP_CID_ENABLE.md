# external_wired.ip.dhcp.cid_enable



This command determines if DHCP (option 61) on the external wired print server is turned on or off.


**Setvar**


To instructs the printer to set the status of the client identifier of the external wired print server:

```
       ! U1 setvar "external_wired.ip.dhcp.cid_enable" "value"

```

**Values**

              - `"off"` client identifier is turned off

              - `"on"` client identifier is turned on

**Default**
```
          "off"

```

**Getvar**


To instruct the printer to respond with the status of the client identifier of the external wired print server:

```
       ! U1 getvar "external_wired.ip.dhcp.cid_enable"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "external_wired.ip.dhcp.cid_enable" "off"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"off"` .


1120




SGD Network Commands
# internal_wired.ip.dhcp.option12



This command specifies if the DHCP option 12 (host name) is on or off in the discovery packet that is sent
from the internal wired print server.


**Setvar**


To instruct the printer to set the DHCP option 12 (host name) in the discovery packet of the internal wired
print server:

```
       ! U1 setvar "internal_wired.ip.dhcp.option12" "value"

```

**Values**

              - `"on"` turns on option 12

              - `"off"` turns off option 12

**Default**
```
          "on"

```

**Getvar**


To retrieve the status of the DHCP option 12 (host name) in the discovery packet of the internal wired print
server:

```
       ! U1 getvar "internal_wired.ip.dhcp.option12"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "internal_wired.ip.dhcp.option12" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1183


SGD Network Commands
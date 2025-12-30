# ip.dhcp.option12



This command specifies if the DHCP option 12 (host name) is on or off in the discovery packet that is sent
from the wireless print server.


**Setvar**


To instructs the printer to set the DHCP option 12 (host name) in the discovery packet that is sent from the
wireless print server:

```
       ! U1 setvar "ip.dhcp.option12" "value"

```

**Values**

              - `"on"` turns on option 12

              - `"off"` turns off option 12

**Default**
```
          "on"

```

**Getvar**


To retrieve the status of the DHCP option 12 (host name) in the discovery packet that is sent from the
wireless print server:

```
       ! U1 getvar "ip.dhcp.option12"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.dhcp.option12" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1236


SGD Network Commands
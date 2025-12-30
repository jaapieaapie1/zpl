# wlan.ip.dhcp.option12



This command specifies if the DHCP option 12 (host name) is on or off in the discovery packet that is sent
from the wireless print server.


**Setvar**


To enable or disable the DHCP option 12:

```
       ! U1 setvar "wlan.ip.dhcp.option12" "value"

```

**Values**

              - `"on"` turns on option 12

              - `"off"` turns off option 12

**Default**
```
          "on"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.ip.dhcp.option12"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "wlan.ip.dhcp.option12" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1417


SGD Network Commands
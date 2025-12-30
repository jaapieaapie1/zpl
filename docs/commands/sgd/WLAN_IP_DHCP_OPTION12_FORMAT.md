# wlan.ip.dhcp.option12_format



This command specifies the format of the option 12 value to be used in the discovery packet of the
wireless print server.


**Setvar**


To set the format of option 12 value to be used in the discovery packet of the wireless print server:

```
       ! U1 setvar "wlan.ip.dhcp.option12_format" "value"

```

**Values**


String


**Default**
```
          ""

```

**Getvar**


To retrieve the format of option 12 value to be used in the discovery packet of the wireless print server:

```
       ! U1 getvar "wlan.ip.dhcp.option12_format"

```

**Example**

This `setvar` example shows configuring the `wlan.ip.dhcp.option12_format` to the value contained
in the `device.friendly_name` .

It is necessary to surround the SGD entry to be used as source for the data with the < and > characters.

```
       ! U1 setvar "wlan.ip.dhcp.option12_format" "<device.friendly_name>"

```

To further explain, if the above command was issued and the value currently stored in the
`device.friendly_name` parameter was `"ShipPrinter"`, then the response to the following
command would be `"ShipPrinter"` :

```
       ! U1 getvar "wlan.ip.dhcp.option12_value"

```

1418


SGD Network Commands
# wlan.ip.dhcp.option12_value



This command retrieves the actual value which will be used in the discovery packet of the wireless print
server.


**Getvar**


To return the actual value which will be used in the discovery packet of the wireless print server:

```
       ! U1 getvar "wlan.ip.dhcp.option12_value"

```

**Example**

This `setvar` example shows configuring the `wlan.ip.dhcp.option12_format` to the value contained
in the `device.friendly_name` .

It is necessary to surround the SGD entry to be used as source for the data with the < and > characters.

```
       ! U1 setvar "wlan.ip.dhcp.option12_format" "<device.friendly_name>"

```

To further explain, if the above command was issued and the value currently stored in the
`device.friendly_name` parameter was `"ShipPrinter`, then the response to following command
would be `"ShipPrinter` :

```
       ! U1 getvar "wlan.ip.dhcp.option12_value"

```

1419


SGD Network Commands
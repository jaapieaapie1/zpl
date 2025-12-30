# internal_wired.ip.dhcp.option12_value



This command retrieves the actual value which will be used in the discovery packet of the internal wired
print server.


**Getvar**


To retrieve the actual value which will be used in the discovery packet of the internal wired print server:

```
       ! U1 getvar "internal_wired.ip.dhcp.option12_value"

```

**Example**

This `setvar` example shows configuring the internal_wired.ip.dhcp.option12_format to the value
contained in the device.friendly_name.


It is necessary to surround the SGD entry to be used as source for the data with the < and > characters.

```
       ! U1 setvar "internal_wired.ip.dhcp.option12_format"
       "<device.friendly_name>"

```

To further explain, if the above command was issued and the value currently stored in the
device.friendly_name parameter was `"`, then the response to following command would be
`"ShipPrinter"` :

```
       ! U1 getvar "internal_wired.ip.dhcp.option12_value"

```

1185


SGD Network Commands
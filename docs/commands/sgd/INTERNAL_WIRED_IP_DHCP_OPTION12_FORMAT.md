# internal_wired.ip.dhcp.option12_format



This command specifies the value which will be used for option 12 (host name) to be used in the DHCP
discovery packet of the internal wired print server.


**Setvar**


To instruct the printer to set value which will be used for option 12 (host name) to be used in the DHCP
discovery packet of the internal wired print server:

```
       ! U1 setvar "internal_wired.ip.dhcp.option12_format" "value"

```

**Values**


0 to 109 alphanumeric characters


**Default**
```
          ""

```

**Getvar**


To retrieve the value which will be used for option 12 (host name) to be used in the DHCP discovery packet
of the internal wired print server:

```
       ! U1 getvar "internal_wired.ip.dhcp.option12_format"

```

**Example**

This `setvar` example shows configuring the internal_wired.ip.dhcp.option12_format to the value
contained in the `device.friendly_name` parameter.

It is necessary to surround the SGD entry to be used as source for the data with the < and > characters.

```
       ! U1 setvar "internal_wired.ip.dhcp.option12_format"
       "<device.friendly_name>"

```

To further explain, if the above command was issued and the value currently stored in the
`device.friendly_name` parameter was `"ShipPrinter"`, then the response to following command
would be `"ShipPrinter"` :

```
       ! U1 getvar "internal_wired.ip.dhcp.option12_value"

```

1184


SGD Network Commands
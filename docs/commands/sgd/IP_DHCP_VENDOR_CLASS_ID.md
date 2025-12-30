# ip.dhcp.vendor_class_id



This command configures the DHCP vendor class ID setting.


**Setvar**


To set the DHCP vendor class ID setting:

```
       ! U1 setvar "ip.dhcp.vendor_class_id" "value"

```

**Values**
```
          Max string length of 64.
```

This command builds a string using the following values: `(device.company_name)`
```
          (device.product_name)–(head.resolution.in_dpi) (device.pnp_option)
```

**Default**


The default varies by printer.


**Getvar**


To retrieve the current DHCP vendor class ID setting:

```
       ! U1 getvar "ip.dhcp.vendor_class_id"

```

1243


SGD Network Commands
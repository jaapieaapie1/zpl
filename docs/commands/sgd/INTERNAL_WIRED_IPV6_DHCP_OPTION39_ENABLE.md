# internal_wired.ipv6.dhcp.option39_enable



This command enables option 39 (to configure the Fully Qualified Domain Name or FQDN) in DHCPv6
communication with a wired connection.


**Setvar**


To enable or disable option 39:

```
       ! U1 setvar "internal_wired.ipv6.dhcp.option39_enable" "value"

```

**Values**

`"on"` or `"off"`

**Default**
```
          "on"

```

**Getvar**


To retrieve the option 39 status:

```
       ! U1 getvar "internal_wired.ipv6.dhcp.option39_enable"

```

**Example**

```
       ! U1 setvar "internal_wired.ipv6.dhcp.option39_enable" "off"

       ! U1 getvar "internal_wired.ipv6.dhcp.option39_enable"

```

**Result**

```
          "off"

```

1206


SGD Network Commands
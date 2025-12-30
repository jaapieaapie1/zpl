# internal_wired.ipv6.address_type



This command controls how the interface gets IPv6 address(es). When set to `"auto"`, the Router
Advertisement packets determine the usage of SLAAC and/or DHCP.


**Setvar**


To set the active network address type:

```
       ! U1 setvar "internal_wired.ipv6.address_type" "value"

```

**Values**

`"static"` or `"auto"`

**Default**
```
          "auto"

```

**Getvar**


To retrieve the address type:

```
       ! U1 getvar "internal_wired.ipv6.address_type"

```

**Example**

This example changes the address type to `"static"` .

```
       ! U1 setvar "internal_wired.ipv6.address_type" "static"

       ! U1 getvar "internal_wired.ipv6.address_type"

```

**Result**

```
          "static"

```

1202


SGD Network Commands
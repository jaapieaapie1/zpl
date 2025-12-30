# ip.address_mode



This command sets the IP protocol value for the printer.


**Setvar**


To set the IP protocol:

```
       ! U1 setvar "ip.address_mode" "value"

```

**Values**

`"ipv4"`, `"ipv6"`, `"all"`

**Default**
```
          "ipv4"

```

**Getvar**


To retrieve the IP protocol value:

```
       ! U1 getvar "ip.address_mode"

```

**Example**

```
       ! U1 setvar "ip.address_mode" "ipv6"

       ! U1 getvar "ip.address_mode"

```

**Result**

```
          "ipv6"

```

1217


SGD Network Commands
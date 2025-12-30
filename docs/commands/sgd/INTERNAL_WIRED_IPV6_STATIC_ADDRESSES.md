# internal_wired.ipv6.static.addresses



This command specifies the IPv6 address(es) to be used when `internal_wired.ipv6.address_type`
is set to `"static"` . Up to three addresses may be set, separated by commas. The command returns `"::"`
when no static address has been set.


**Setvar**


To set the IPv6 address(es):

```
       ! U1 setvar "internal_wired.ipv6.static.addresses" "value"

```

where `"value"` is a string of up to 152 characters.


**Getvar**


To retrieve the list of IPv6 address(es):

```
       ! U1 getvar "internal_wired.ipv6.static.addresses"

```

**Example**


To set three addresses:

```
       ! U1 setvar "internal_wired.ipv6.static.addresses"
       "fc04:1795::fe94:1704/32,fd04:1795::207:4dff:fe94:1704/64,fd04:1796::e0b/64"

       ! U1 getvar "internal_wired.ipv6.static.addresses"

```

**Result**

```
          "fc04:1795::fe94:1704/32,fd04:1795::207:4dff:fe94:1704/64,
          fd04:1796::e0b/64"

```

1211


SGD Network Commands
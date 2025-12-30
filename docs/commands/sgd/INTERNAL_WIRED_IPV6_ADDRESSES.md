# internal_wired.ipv6.addresses



This command returns a list of up to ten IPv6 addresses in use by the printer with a wired connection when
`internal_wired.ipv6.address_type` is set to `auto` . The command returns `"::"` when no address
has been set.


**Getvar**


To retrieve the IPv6 addresses:

```
       ! U1 getvar "internal_wired.ipv6.addresses"

```

1201


SGD Network Commands
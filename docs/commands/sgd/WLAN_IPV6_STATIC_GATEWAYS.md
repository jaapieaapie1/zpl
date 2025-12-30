# wlan.ipv6.static.gateways



This command sets the IPv6 gateway to be used when `wlan.ipv6.address_type` is set to `"static"` .
Only one gateway is supported. The command returns `"::"` when no static address has been set.


**Setvar**


To set the gateway:

```
       ! U1 setvar "wlan.ipv6.static.gateways" "value"

```

where `"value"` is a string of up to 50 characters. To clear a previously set gateway, use `""` or `"::"` .


**Getvar**


To retrieve the active network address type:

```
       ! U1 getvar "wlan.ipv6.static.gateways"

```

**Example**

To set the gateway to `"fe80::202:b3ff:febf:9d18"` :

```
       ! U1 setvar "wlan.ipv6.static.gateways" "fe80::202:b3ff:febf:9d18"

       ! U1 getvar "wlan.ipv6.static.gateways"

```

**Result**

```
          "fe80::202:b3ff:febf:9d18"

```

1448


SGD Network Commands
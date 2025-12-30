# wlan.ip.dns.domain



This command sets the value for DNS domains in a WLAN network. Up to five domains are allowed to be
set.


**Setvar**

To set values to be used by the device as DNS domains when `wlan.ipv6.address_type` is `"static"`
(IPv6) or `wlan.ip.protocol` is `"permanent"` (IPv4):

```
       ! U1 setvar "wlan.ip.dns.domain" "zebra.com zebra-lab.lan.com"

```

**Getvar**


To retrieve a space-delimited list of domains currently in use by the device, up to five domains. The list
will contain a mixture of domains received from DHCPv4, DHCPv6, and user-set values depending upon
address acquisition settings, using the following rules:


          - IPv4 sources are the priority.


          - If IPv4 the source provides five domains, there will only be IPv4-sourced domains.


          - If there are slots to fit more domains, they will be filled with IPv6 sources.

```
       ! U1 getvar "wlan.ip.dns.domain"

```

**NOTE:** Retrieved values are always the values currently in use, which may not match values that
were just set, depending on if you are using a static or auto address type.


**Example**

The following example sets `wlan.ip.dns.domain` to the domains `zebra.com` and `zebra-`
`lab.lan.com` .

These values will be retrieved immediately if `internal_wired.ipv6.address_type` is `"static"`
(IPv6) or `wlan.ip.protocol` is `"permanent"` (IPv4).

```
       ! U1 setvar "wlan.ip.dns.domain" "zebra.com zebra-lab.lan.com"

       ! U1 getvar "wlan.ip.dns.domain"

```

**Result**

```
          "zebra.com zebra-lab.lan.com"

```

1423


SGD Network Commands
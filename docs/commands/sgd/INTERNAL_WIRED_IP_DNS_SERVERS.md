# internal_wired.ip.dns.servers



This command sets a space-delimited list of the domain name servers from an internal wired print server.
Up to three addresses may be set. Both IPv4 and IPv6 are supported.


**Setvar**


To set the list of DNS internal wired print servers, specify a list of space-delimited IP addresses, separated
by spaces, to be used as DNS servers when `internal_wired.ipv6.address_type` is `"static"`
(IPv6) or `internal_wired.ip.protocol` is `"permanent"` (IPv4).

```
       ! U1 setvar "internal_wired.ip.dns.servers" "value"

```

**Getvar**


To retrieve a space-delimited list of IP address(es) of DNS server(s), up to three, that are currently in use by
the device. The values to be used are a combination of addresses received from DHCPv4, DHCPv6, and
user-set values using the following rules:


          - At least one spot will be allocated to any enabled IP version.


          - IPv4 addresses will take at least two slots if at least two IPv4 addresses are provided.


          - User-set values will be included if static/permanent addresses are used.


**NOTE:** Retrieved values are always the values currently in use, which may not match values that
were just set, depending on if you are using a static or auto address type.

```
       ! U1 getvar "internal_wired.ip.dns.servers"

```

**Example**

The following example sets `internal_wired.dns.servers` to `2001::123:4567:89ab:0:cdef` .

This value will be retrieved immediately if `internal_wired.ipv6.address_type` is `"static"` (IPv6)
or `internal_wired.ip.protocol` is `"permanent"` (IPv4).

```
       ! U1 setvar "internal_wired.ip.dns.servers" "2001::123:4567:89ab:0:cdef"

       ! U1 getvar "internal_wired.ip.dns.servers"

```

**Result**

```
          "2001::123:4567:89ab:0:cdef"

```

1189


SGD Network Commands
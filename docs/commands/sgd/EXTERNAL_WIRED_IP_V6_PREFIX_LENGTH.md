# external_wired.ip.v6.prefix_length



This command retrieves the IPv6 address prefix length of the SEH wired print server.


**Getvar**


To retrieve the IPv6 address prefix length of the SEH wired print server:

```
       ! U1 getvar "external_wired.ip.v6.prefix_length"

```

**Values**


character set = 0-9 (3-character maximum)


              - SEH print server model PS105-Z with firmware version V60.16.5Z or V53.16.5Z and later.


              - SEH print server model with firmware version V60.16.5Z or V53.16.5Z and later.
PS102-Z


**Example**

In this example, the `getvar` returns the IPv6 address prefix length of the wired print server.

```
       ! U1 getvar "external_wired.ip.v6.prefix_length"

```

1132


SGD Network Commands
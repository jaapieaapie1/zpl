# external_wired.ip.v6.addr



This command retrieves the IPv6 address of the SEH wired print server. This command is only supported
on SEH print server models PS105-Z and with firmware version V60.16.5Z or V53.16.5Z and later.
PS102-Z

**Getvar**


To retrieve the IPv6 address of the SEH wired print server:

```
       ! U1 getvar "external_wired.ip.v6.addr"

```

**Values**


8 group of four hexadecimal digits with a colon delimiter


character set A-F or 0-9 with a 39-character maximum


              - SEH print server model PS105-Z with firmware version V60.16.5Z or V53.16.5Z and later.


              - SEH print server model with firmware version V60.16.5Z or V53.16.5Z and later.
PS102-Z


**Example**

In this example, the `getvar` returns the IPv6 address of the wired print server.

```
       ! U1 getvar "external_wired.ip.v6.addr"

```

1130




SGD Network Commands
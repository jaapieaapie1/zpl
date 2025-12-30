# ip.dhcp.lease.server



This command retrieves the address of the server that provided the DHCP lease on the wireless print
server.


**Getvar**


To retrieve the address of the server that provided the DHCP lease on the wireless print server:

```
       ! U1 getvar "ip.dhcp.lease.server"

```

**Example**

In this example, the `getvar` retrieves the server that provided the DHCP lease on the wireless print server.

```
       ! U1 getvar "ip.dhcp.lease.server"
       "10.3.5.1"

```

1232


SGD Network Commands
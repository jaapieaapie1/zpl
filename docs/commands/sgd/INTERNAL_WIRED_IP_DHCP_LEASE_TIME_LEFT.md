# internal_wired.ip.dhcp.lease.time_left



This command retrieves the time (in seconds) left in the current DHCP lease on the internal wired print
server.


**Getvar**


To retrieve the time (in seconds) left in the current DHCP lease on the internal wired print server:

```
       ! U1 getvar "internal_wired.ip.dhcp.lease.time_left"

```

**Example**

In this example, the `getvar` retrieves the time left in the current DHCP lease on the wired internal print
server.

```
       ! U1 getvar "internal_wired.ip.dhcp.lease.time_left" "10.3.1.98"

```

1182


SGD Network Commands
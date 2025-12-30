# ip.dhcp.lease.last_attempt



This command retrieves the time from the DHCP server of when the last DHCP request was sent.


**Getvar**


To retrieve the last time a DHCP request was sent:

```
       ! U1 getvar "ip.dhcp.lease.last_attempt"

```

**Example**

In this example, the `getvar` retrieves the last time a DHCP request was sent to the wireless print server.

```
       ! U1 getvar "ip.dhcp.lease.last_attempt"

```

1231


SGD Network Commands
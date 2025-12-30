# ip.dhcp.ntp.received_servers



Returns the list of Network Time Protocol (NTP) server IP Addresses received via DHCP.


**Getvar**


To return the current setting value:

```
       ! U1 getvar "ip.dhcp.ntp.received_servers"

```

**Values**


A comma-separated list of IP address. The maximum number of servers listed is three (3).


**Default**
```
          ""

```

**Example**

```
       "10.4.3.24,172.30.16.52"

```

1235


SGD Network Commands
# wlan.ip.arp_interval



This print server setting allows you to specify the ARP (Address Resolution Protocol) interval or the ARP
cache time out for the wireless print server.


**Setvar**


To instruct the printer to respond with the ARP interval or the ARP cache time out value for the wireless
print server:

```
       ! U1 setvar "wlan.ip.arp_interval" "value"

```

**Values**
`"0"` to `"30"` seconds

**Default**
```
          "0"

```

**Getvar**


To respond with the ARP interval or the ARP cache time out value (in seconds) for the wireless print server:

```
       ! U1 getvar "wlan.ip.arp_interval"

```

**Example**

This `setvar` example shows the value set to `"0"` .

```
       ! U1 setvar "wlan.ip.arp_interval" "0"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"0"` .


1404


SGD Network Commands
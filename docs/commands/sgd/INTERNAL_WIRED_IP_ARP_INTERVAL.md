# internal_wired.ip.arp_interval



This print server setting allows you to specify the ARP (Address Resolution Protocol) interval or the ARP
cache time out for the internal wired print server.


**Setvar**


To instruct the printer to change the ARP interval or the ARP cache time out for the internal wired print
server:

```
       ! U1 setvar "internal_wired.ip.arp_interval" "value"

```

**Values**

`"0"`         - `"30"`

**Default**
```
          "0"

```

**Getvar**


To instruct the printer to respond with the ARP interval or the ARP cache time out value for the internal
wired print server:

```
       ! U1 getvar "internal_wired.ip.arp_interval"

```

**Example**

This `setvar` example shows the value set to `"0"` .

```
       ! U1 setvar "internal_wired.ip.arp_interval" "0"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"0"` .


1170




SGD Network Commands
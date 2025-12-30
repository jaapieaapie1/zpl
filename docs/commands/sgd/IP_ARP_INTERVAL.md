# ip.arp_interval



This printer setting allows you to specify the ARP (Address Resolution Protocol) interval or the ARP cache
time out.


**Setvar**


To instruct the printer to change the ARP interval or the ARP cache time out:

```
       ! U1 setvar "ip.arp_interval" "value"

```

**Values**

`"0"` through `"30"`

**Default**
```
          "0"

```

**Getvar**


To instruct the printer to respond with the ARP interval or the ARP cache time out value in seconds:

```
       ! U1 getvar "ip.arp_interval"

```

**Example**

This `setvar` example shows the value set to `"0"` .

```
       ! U1 setvar "ip.arp_interval" "0"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"0"` .


1218


SGD Network Commands
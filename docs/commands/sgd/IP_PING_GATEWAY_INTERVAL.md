# ip.ping_gateway_interval



Determines the interval in minutes at which to send ICMP PING packets to the default gateway.


**Setvar**


To set the command:

```
       ! U1 setvar "ip.ping_gateway_interval" "value"

```

**Value**

An integer from `"0"` to `"30"` . `"0"` is disabled

**Default**
```
          "0"

```

**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "ip.ping_gateway_interval"

```

1294


SGD Network Commands
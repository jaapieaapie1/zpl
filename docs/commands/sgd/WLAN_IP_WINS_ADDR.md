# wlan.ip.wins.addr



Specifies the WINS server address. If WINS Addressing is done via DHCP, then this value will automatically
be filled by the DHCP server.


**Setvar**


To set the WINS server address:

```
       ! U1 setvar "wlan.ip.wins.addr" "value"

```

**Values**


A valid IP address


**Default**
```
          "0.0.0.0"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.ip.wins.addr"

```

1435


SGD Network Commands
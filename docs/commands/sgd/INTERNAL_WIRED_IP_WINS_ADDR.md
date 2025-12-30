# internal_wired.ip.wins.addr



Sets or returns the current WINS server address. If WinsAddressing is DHCP, then this will automatically be
filled by the DHCP server.


**Setvar**


To set the current WINS server address:

```
       ! U1 setvar "internal_wired.ip.wins.addr" "value"

```

**Values**


A valid WINS IP address.


**Default**
```
          "0.0.0.0"

```

**Getvar**


To return the WINS server address:

```
       ! U1 getvar "internal_wired.ip.wins.addr"

```

**Result**


The current WINS server address


1199


SGD Network Commands
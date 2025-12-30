# wlan.ip.wins.permanent_source



Specifies if the source of the WINS address is DHCP or if it is set manually.


**Setvar**


To set the source of the WINS address:

```
       ! U1 setvar "wlan.ip.wins.permanent_source" "value"

```

**Values**

`"off":` uses DHCP assigned WINS address

`"on":` manually sets WINS address (set via `wlan.ip.wins.addr` )

**Default**
```
          "off"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.ip.wins.permanent_source"

```

1436


SGD Network Commands
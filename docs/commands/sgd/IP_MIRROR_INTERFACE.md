# ip.mirror.interface



Determines the interface over which Mirror will operate.


**Setvar**


To set the interface over which Mirror will operate:

```
       ! U1 setvar "ip.mirror.interface" "value"

```

**Values**

`"both"` internal or external wired and WLAN

`"wired"` internal or external wired

`"wireless"` WLAN

**Default**
```
          "both"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "ip.mirror.interface"

```

1278


SGD Network Commands
# wlan.11n.20mhz_only



This command forces 20 MHz operation only in printers with 802.11n radios.


**Setvar**


To force 20 MHz operation only in printers with 802.11n radios:

```
       ! U1 setvar "wlan.11n.20mhz_only" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**


To return the current setting for 20 MHz operation only:

```
       ! U1 getvar "wlan.11n.20mhz_only"

```

**Example**

```
       U1 setvar "wlan.11n.20mhz_only" "on"

```

1363


SGD Network Commands
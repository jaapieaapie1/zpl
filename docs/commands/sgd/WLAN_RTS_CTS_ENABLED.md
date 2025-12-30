# wlan.rts_cts_enabled



Enables the RTS/CTS HT protection frames when configuring a WLAN connection, preventing interference
with other nearby 802.11 signals. If the protection frames are not enabled, the WLAN radio will use CTS-toself.


**Setvar**

```
       ! U1 setvar "wlan.rts_cts_enabled" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**

```
       ! U1 getvar "wlan.rts_cts_enabled"

```

**Values**

              - `"on"`

              - `"off"`


1477


SGD Network Commands
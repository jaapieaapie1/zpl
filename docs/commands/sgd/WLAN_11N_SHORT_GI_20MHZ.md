# wlan.11n.short_gi_20mhz



This command enables/disables short Guard Interval (GI) in 20 mhz mode.


**Setvar**


To enable or disable short Guard Interval in 20 mhz mode:

```
       ! U1 setvar "wlan.11n.short_gi_20mhz" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.11n.short_gi_20mhz"

```

**Example**

```
       ! U1 setvar "wlan.11n.short_gi_20mhz" "on"

```

1368


SGD Network Commands
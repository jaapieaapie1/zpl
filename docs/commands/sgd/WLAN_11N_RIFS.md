# wlan.11n.rifs



This command enable/disables Reduced Interframe Space (RIFS) in 802.11n radio communications.


**Setvar**


To enable or disable RIFS in 802.11n:

```
       ! U1 setvar "wlan.11n.rifs" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**


To return the current setting for RIFS:

```
       ! U1 getvar "wlan.11n.rifs"

```

**Example**

```
       ! U1 setvar "wlan.11n.rifs" "on"

```

1366


SGD Network Commands
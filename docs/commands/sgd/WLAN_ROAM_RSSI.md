# wlan.roam.rssi



This command allows you to specify the absolute value of the negative dBm for the RSSI threshold, which
is the point at which the radio will start the roaming algorithm.


**Setvar**


This command sets the RSSI threshold value.

```
       ! U1 setvar "wlan.roam.rssi" "value"

```

**Values**
`"60"` to `"125"`

**Default**
```
          "74"

```

**Getvar**


This command retrieves the absolute value of the negative dBM for the RSSI threshold.

```
       ! U1 getvar "wlan.roam.rssi"

```

**Example**

In this example, the `setvar` sets the RSSI threshold value to -80 dBm.

```
       ! U1 setvar "wlan.roam.rssid" "80"

```

1475


SGD Network Commands
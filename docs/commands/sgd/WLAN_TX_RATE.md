# wlan.tx_rate



Use this command to specify the wireless transmit rate.


**Setvar**


To set the wireless transmit rate:

```
       ! U1 setvar "wlan.tx_rate" "value"

```

**Values**

`"1"`, `"2"`, `"5.5"`, `"11"`, `"all"`

**Default**
```
          "all"

```

**Getvar**


To respond with the wireless transmit rate:

```
       ! U1 getvar "wlan.tx_rate"

```

**Example**

This `setvar` example shows the value set to `"all"` .

```
       ! U1 setvar "wlan.tx_rate" "all"

```

When the `setvar` value is set to `"all"`, the `getvar` result is `"all"` .


1493


SGD Network Commands
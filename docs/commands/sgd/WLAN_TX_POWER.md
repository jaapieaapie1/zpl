# wlan.tx_power



Use this command to specify the wireless transmit power.


**Setvar**


To set the wireless transmit power:

```
       ! U1 setvar "wlan.tx_power" "value"

```

**Values**

Decimal values of `"1"`, `"5"`, " `20"`, `"30"`, `"50"`, `"100"`

**Default**
```
          "100"

```

**Getvar**


To return with the wireless transmit power value:

```
       ! U1 getvar "wlan.tx_power"

```

**Example**

This `setvar` example shows the value set to `"100"` .

```
       ! U1 setvar "wlan.tx_power" "100"

```

When the `setvar` value is set to `"100"`, the `getvar` result is `"100"` .


1492


SGD Network Commands
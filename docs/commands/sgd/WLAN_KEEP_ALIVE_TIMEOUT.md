# wlan.keep_alive.timeout



This printer setting manages the interval at which the LSAP (link service access point) packet is sent.


**Setvar**

To configure the frequency at which the printer sends the `wlan.keep_alive packet` :

```
       ! U1 setvar "wlan.keep_alive.timeout" "value"

```

**Values**
`"5"` to `"300"` seconds

**Default**
```
          "15"

```

**Getvar**

To respond with the `wlan.keep_alive.timeout` interval value:

```
       ! U1 getvar "wlan.keep_alive.timeout"

```

**Example**

This `setvar` example shows the value set to `"15"` .

```
       ! U1 setvar "wlan.keep_alive.timeout" "15"

```

When the `setvar` value is set to `"15"`, the `getvar` result is `"15"` .


1450




SGD Network Commands
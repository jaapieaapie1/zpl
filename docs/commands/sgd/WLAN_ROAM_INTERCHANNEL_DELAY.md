# wlan.roam.interchannel_delay



This command sets how long of a delay before scanning the next channel when roaming.


**Setvar**


This command sets how long of a delay before scanning the next channel when roaming. The values are in
milliseconds.

```
       ! U1 setvar "wlan.roam.interchannel_delay" "value"

```

**Values**
`"0"` to `"30000"`

**Default**
```
          "400"

```

**Getvar**


This command retrieves the current set delay time before scanning the next channel when roaming.

```
       ! U1 getvar "wlan.roam.interchannel_delay"

```

**Example**

This `setvar` example shows the value set to `"400"` .

```
       ! U1 setvar "wlan.roam.interchannel_delay" "400"

```

The `getvar` result returns the current `setvar` value. In this example, the `getvar` result is `"400"` .


1469


SGD Network Commands
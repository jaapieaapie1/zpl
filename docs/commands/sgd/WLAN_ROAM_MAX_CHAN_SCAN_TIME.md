# wlan.roam.max_chan_scan_time



This command sets how long the radio waits on a channel looking for probe responses.


**Setvar**


This command sets how long the radio waits on a channel looking for probe responses. The values are in
milliseconds.

```
       ! U1 setvar "wlan.roam.max_chan_scan_time" "value"

```

**Values**
`"10"` to `"500"`

**Default**
```
          "100"

```

**Getvar**


This command retrieves the current setting for how long the radio waits on a channel looking for probe
responses.

```
       ! U1 getvar "wlan.roam.max_chan_scan_time"

```

**Example**

This `setvar` example shows the value set to `"100"` .

```
       ! U1 setvar "wlan.roam.max_chan_scan_time" "100"

```

The `getvar` result returns the current `setvar` value. In this example, the `getvar` result is `"100"` .


1471


SGD Network Commands
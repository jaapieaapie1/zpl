# wlan.roam.max_fail



This command determines teh number of consecutive tx packet failures, at which point th radio should start
its roaming algorithm.


**Setvar**


This command sets the max_fail threshold value.

```
       ! U1 setvar "wlan.roam.max_fail" "value"

```

**Values**
`"2"` to `"75"` inclusive

**Default**
```
          "10"

```

**Getvar**


This command returns the number for the max_fail threshold.

```
       ! U1 getvar "wlan.roam.max_fail"

```

**Example**

In this example, the `setvar` sets the max_fail threshold value to 30 packets.

```
       ! U1 setvar "wlan.roam.max_fail" "30"

```

1472


SGD Network Commands
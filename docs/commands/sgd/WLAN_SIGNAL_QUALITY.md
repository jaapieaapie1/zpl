# wlan.signal_quality



This command instructs the printer to return the current signal quality of the wireless network. Values
below 40% represent a very poor signal quality, and radio communication is not reliable.


**Getvar**


To return the current signal quality of the wireless network:

```
       ! U1 getvar "wlan.signal_quality"

```

**Example**

In this example, the `getvar` result is the current signal_quality value.

```
       ! U1 getvar "wlan.signal_quality"

```

1488


SGD Network Commands
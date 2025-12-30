# wlan.signal_strength



This command returns the signal strength of the connection to the access point as a percentage value
between zero (not connected) and 100 (strongest signal). Values below 40% represent a very poor signal
and radio communication is not reliable.


**Getvar**


To respond with the current signal strength:

```
       ! U1 getvar "wlan.signal_strength"

```

**Example**

In this example, the `getvar` result is `"93"` .

```
       ! U1 getvar "wlan.signal_strength"

```

1489


SGD Network Commands
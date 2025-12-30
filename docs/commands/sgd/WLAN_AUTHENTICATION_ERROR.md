# wlan.authentication_error



Reports the last error that occurred during the WLAN authentication process.


**Getvar**


To report the last error that occurred during the WLAN authentication process:

```
       ! U1 getvar "wlan.authentication_error"

```

**Values**

              - `"none"` the authentication was successful.

              - `"timed out"` the authentication did not succeed in the allotted time.


1383


SGD Network Commands
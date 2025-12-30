# wlan.leap_mode



This printer setting refers to Cisco LEAP (Lightweight Extensible Authentication Protocol). LEAP provides
secure mutual authentication for a wireless client through a Cisco Aironet Access Point, based on
user information stored on a backend RADIUS (Remote Authentication in Dial-Up User Service) /AAA
(Authentication, Authorization, and Accounting) server.


**NOTE:**


          - This command is only supported on printers using firmware Vxx.18.xx or earlier.


          - This command is not supported on units with a Frequency Hopping Spread Spectrum (FHSS)
radio.


**Setvar**

To turn the LEAP mode `"on"` or `"off":`

```
       ! U1 setvar "wlan.leap_mode" "values"

```

**Values**

`"off"` disables LEAP mode

`"on"` enables LEAP mode

**Default**
```
          "off"

```

**Getvar**


To respond with the LEAP mode:

```
       ! U1 getvar "wlan.leap_mode"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "wlan.leap_mode" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1456


SGD Network Commands
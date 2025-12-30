# device.feature.802_11ax



This command returns the status of the 802.11ax radio.


**Getvar**


To return the status of the 802.11ax radio:

```
       ! U1 getvar "device.feature.802_11ax"

```

**Values**

              - `not available` if the printer model does not support the 802.11ax feature.

              - `not present` if the printer model supports the 802.11ax feature, but the feature is not installed
on the printer.

              - `present` if the feature is installed on the printer.


694


SGD Printer Commands
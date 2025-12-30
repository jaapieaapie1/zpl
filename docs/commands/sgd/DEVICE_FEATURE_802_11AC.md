# device.feature.802_11ac



This command returns information on the 802.11AC radio status.


**Getvar**


To return the current setting:

```
       ! U1 getvar "device.feature.802_11ac"

```

**Result**

              - `"not present"` if the printer model supports an 802.11ac option but the printer does not have
the feature installed.

              - `"not available"` if the printer model does not support an 802.11ac feature option.

              - `"present"` if the printer has an 802.11ac radio installed.


693


SGD Printer Commands
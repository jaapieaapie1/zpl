# device.feature.nfc



Indicates if the printer supports the optional Active Near Field Communication (NFC) feature, and if it is
currently installed.


**Getvar**


To see if the printer supports the optional Active Near Field Communication (NFC) feature, and if it is
currently installed:

```
       ! U1 getvar "device.feature.nfc"

```

**Values**

              - `"not available"` active NFC is not supported.

              - `"not present"` active NFC is supported, but no reader is installed.

              - `"present"` active NFC is supported with a reader is installed.

**Default**
NA


691


SGD Printer Commands
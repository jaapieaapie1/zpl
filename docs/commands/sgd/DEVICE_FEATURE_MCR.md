# device.feature.mcr



Indicates if the magnetic card reader is installed and available.


**Getvar**


To return if the magnetic card reader is installed and available:

```
       ! U1 getvar "device.feature.mcr"

```

**Values**

              - `"not available"` the magnetic card reader is not available on the printer

              - `"not present"` the magnetic card reader is available but not installed

              - `"present"` the magnetic card reader is both available and installed on the printer

**Default**
NA


690




SGD Printer Commands
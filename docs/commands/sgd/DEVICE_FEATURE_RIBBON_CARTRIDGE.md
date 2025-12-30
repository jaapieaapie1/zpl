# device.feature.ribbon_cartridge



Indicates if the printer can accept a ribbon cartridge, and if so, if one is installed.


**Getvar**


To return if a ribbon cartridge is installed or not:

```
       ! U1 getvar "device.feature.ribbon_cartridge"

```

**Result**

`"not available"` the ribbon cartridge is not available on the platform

`"not present"` the printer is capable of accepting a ribbon cartridge, but one is not currently
installed

`"present"` a ribbon cartridge is installed


692


SGD Printer Commands
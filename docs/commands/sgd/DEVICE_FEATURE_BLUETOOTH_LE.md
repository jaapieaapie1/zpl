# device.feature.bluetooth_le



Indicates whether or not the printer supports Bluetooth LE.


**Getvar**


To return if the printer supports Bluetooth LE:

```
       ! U1 getvar "device.feature.bluetooth_le"

```

**Values**

              - `"present"` Bluetooth LE radio is installed

              - `"not present"` Bluetooth LE radio is not installed

              - `"not available"` Bluetooth LE radio is not available on this printer


689


SGD Printer Commands
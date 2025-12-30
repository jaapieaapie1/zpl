# usb.device.device_id_string



This command retrieves the manufacturer assigned IEEE1284 Device Identification string for USB devices.


**Getvar**


To retrieve the device ID string:

```
       ! U1 getvar "usb.device.device_id_string"

```

**Example**

```
       ! U1 getvar "usb.device.device_id_string"

```

**Result**

```
          "MANUFACTURER:Zebra Technologies ;COMMAND SET:ZPL;MODEL:ZTC
          ZT220-200dpi ZPL;CLASS:PRINTER;OPTIONS:XML;"

```

1011


SGD Printer Commands
# usb.host.hid_count



This command displays the number of USB Human Interface Devices (HIDs) connected to the printer.


[Refer to http://www.usb.org](http://www.usb.org) for further details on USB device types.


**Getvar**


To retrieve the number of USB HIDs connected to the printer:

```
       ! U1 getvar "usb.host.hid_count"

```

**Result**


When no devices are attached:

```
          ! U1 getvar "usb.host.hid_count"
          "0"

```

1023


SGD Printer Commands
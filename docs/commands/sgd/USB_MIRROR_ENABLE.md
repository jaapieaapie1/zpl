# usb.mirror.enable



This command enables or disables the ability to perform mirroring of a USB device.


**Setvar**


To enable or disable the USB device mirroring ability:

```
       ! U1 setvar "usb.mirror.enable" "value"

```

**Values**

              - `"on"` Mirroring is enabled

              - `"off"` Mirroring is disabled

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "usb.mirror.enable"

```

1034


SGD Printer Commands
# usb.mirror.auto



This command determines if mirroring happens automatically when a USB device is inserted in the printer.
To use this function, the setting for `usb.mirror.enable` must be `"on"` .


**Setvar**


To set the automatic mirroring of a USB device:

```
       ! U1 setvar "usb.mirror.auto" "value"

```

**Values**

              - `"on"` Mirroring occurs automatically when the USB device is inserted and
`usb.mirror.enable` is on.

              - `"off"` Mirroring does not occur automatically when the USB device is inserted.

              - `"prompt"` The printer gives you the option to start or abort a mirroring process.


**Getvar**


To return the current setting value:

```
       ! U1 getvar "usb.mirror.auto"

```

1033


SGD Printer Commands
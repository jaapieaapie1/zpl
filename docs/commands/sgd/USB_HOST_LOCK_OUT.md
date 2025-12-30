# usb.host.lock_out



This command enables/disables the USB host port.


**Setvar**


To enable or disable the USB port:

```
       ! U1 setvar "usb.host.lock_out" "value"

```

**Values**

              - `"on"` disables the USB host port.

              - `"off"` enables the USB host port.

**Default**
```
          "off"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "usb.host.lock_out"

```

1025


SGD Printer Commands
# usb.host.keyboard_input



This command enables/disables USB keyboard input to the printer, affecting the Print Station user menu
item.


**Setvar**


To enable or disable USB keyboard input to the printer:

```
       ! U1 setvar "usb.host.keyboard_input"

```

**Values**

              - `"on"` means keyboard input is supported.

              - `"off"` means keyboard input is not supported

**Default**
```
          "on"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "usb.host.keyboard_input"

```

1024


SGD Printer Commands
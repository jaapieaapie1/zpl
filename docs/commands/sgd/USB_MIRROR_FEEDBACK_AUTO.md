# usb.mirror.feedback.auto



This command determines if a feedback file is written to the USB device connected to the printer after
mirroring.


**Setvar**


To specify if the feedback file is written to the USB device or not:

```
       ! U1 setvar "usb.mirror.feedback.auto" "value"

```

**Values**

              - `"on"` A feedback file is written to the USB device connected to the printer.

              - `"off"` A feeback file is not written to the USB device connected to the printer.


**Getvar**


To return the current setting value:

```
       ! U1 getvar "usb.mirror.feedback.auto"

```

1036


SGD Printer Commands
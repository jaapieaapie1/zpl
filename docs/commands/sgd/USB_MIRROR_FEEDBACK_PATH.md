# usb.mirror.feedback.path



This command specifies where the feedback file is stored on the USB drive connected to the printer.


**Setvar**


To specify the path of the feedback file stored on the USB drive:

```
       ! U1 setvar "usb.mirror.feedback.path" "value"

```

**Values**


A valid path up to 255 characters


**NOTE:** The path must exist on the USB drive before the printer can write files to it.


**Default**
```
          "zebra/feedback"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "usb.mirror.feedback.path"

```

1038


SGD Printer Commands
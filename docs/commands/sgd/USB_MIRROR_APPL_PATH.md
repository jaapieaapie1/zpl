# usb.mirror.appl_path



This command specifies the path to the location on a USB device from which Mirror files are retrieved. If no
path is specified, then the path is `zebra/appl` .


**Setvar**


To specify the path to the location on a USB device from which Mirror files are retrieved:

```
       ! U1 setvar "usb.mirror.appl_path" "path"

```

**Values**


A valid path up to 255 characters.


**Default**
```
          "zebra/appl"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "usb.mirror.appl_path"

```

1032


SGD Printer Commands
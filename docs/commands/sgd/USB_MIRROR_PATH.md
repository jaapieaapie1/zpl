# usb.mirror.path



This command specifies the root directory on the connected USB drive. The root directory contains the
subdirectories from which files are retrieved during the mirror operation. If no path is specified, then the
path is `/zebra` .

The standard subdirectores to use are:

          - `"<update-root>"/appl`

          - `"<update-root>"/files`

          - `"<update-root>"`

          - `"<feedback-root>"`

See How Mirror Works on page 1685 for a comprehensive overview of mirroring.


**Setvar**


To specify the root directory on the connected USB drive:

```
       ! U1 setvar "usb.mirror.appl_path" "path"

```

**Values**


A valid path up to 255 characters.


**Default**
```
          "zebra"

```

**Getvar**


To retrieve the current setting:

```
       ! U1 getvar "usb.mirror.path"

```

1042


SGD Printer Commands
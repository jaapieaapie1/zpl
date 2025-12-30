# usb.mirror.error_retry



This command specifies the number of times that the USB mirror operation will be repeated if the process
fails.


**Setvar**


To specify the number of times that the USB mirror operation will be repeated:

```
       ! U1 setvar "usb.mirror.error_retry" "value"

```

**Values**

`"0"` to `"65535"`

**Default**
```
          "0"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "usb.mirror.error_retry"

```

1035


SGD Printer Commands
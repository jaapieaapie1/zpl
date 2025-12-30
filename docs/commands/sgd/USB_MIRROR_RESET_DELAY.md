# usb.mirror.reset_delay



This command specifies a number of seconds for the printer to wait after it loads the last of the files in the
`/commands` directory during mirroring.


**Setvar**


To specify the USB mirror reset delay time:

```
       ! U1 setvar "usb.mirror.reset_delay"

```

**Values**

`"0"` to `"900"`

**Default**
```
          "5"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "usb.mirror.reset_delay"

```

1043


SGD Printer Commands
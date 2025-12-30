# device.idle_display_format



Retrieves and sets the front panel’s idle display format.


**NOTE:** This command does not apply to printers with a color-touch display.


**Setvar**


To set the front panel’s idle display format:

```
       ! U1 setvar "device.idle_display_format" "value"

```

**Values**
```
          fw-version,ip-address,mm/dd/yy-24-hr,mm/dd/yy-12-hr,dd/mm/yy-24-hr,dd/
          mm/yy-12-hr
```

**Default**

`"fw-version"` (firmware version)


**Getvar**


To retrieve the front panel’s idle display format:

```
       ! U1 getvar "device.idle_display_format"

```

**Result**
```
          fw-version,ip-address,mm/dd/yy-24-hr,mm/dd/yy-12-hr,dd/mm/yy-24-hr,dd/
          mm/yy-12-h

```

708


SGD Printer Commands
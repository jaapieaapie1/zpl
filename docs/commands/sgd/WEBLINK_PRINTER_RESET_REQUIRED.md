# weblink.printer_reset_required



This command retrieves a `"yes"` or `"no"` value indicating whether any of the weblink settings have been
modified.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Getvar**


To retrieve whether any of the weblink settings are modified:

```
       ! U1 getvar "weblink.printer_reset_required"

```

**Values**

              - `"yes"`

              - `"no"`

**Default**
```
          "no"

```

1352


SGD Network Commands
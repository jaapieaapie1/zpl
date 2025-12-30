# weblink.restore_defaults



This command defaults, and saves, the weblink branch settings. Any value, including an empty string, will
default the weblink branch settings.


**NOTE:** The entire weblink branch of settings will be defaulted and the settings are saved;
however, the weblink connections will not use the new settings until the printer is restarted (e.g.
the `weblink.printer_reset_required` SGD will be `"yes"` after a default).

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To default the weblink branch settings:

```
       ! U1 setvar "weblink.restore_defaults" "value"

```

**Values**


Any value, including an empty string, will default the branch


**Default**


NA


**Do**


To default the weblink branch settings:

```
       ! U1 do "weblink.restore_defaults" "value"

```

**Values**


Any value, including an empty string, will default the branch


**Default**


NA


**Example**


These all default the branch:

```
       ! U1 setvar "weblink.restore_defaults" ""
       ! U1 setvar "weblink.restore_defaults" "foo"
       ! U1 do "weblink.restore_defaults" ""
       ! U1 do "weblink.restore_defaults" "foo"

```

1353


SGD Network Commands
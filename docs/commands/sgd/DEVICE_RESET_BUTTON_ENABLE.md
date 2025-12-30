# device.reset_button_enable



This command disables the reset button on a printer for situations where the reset button might be
accidentally triggered. The default value is `"on"` and the setting is not defaulted during a printer default.


**Setvar**


To disable the printer reset button:

```
       ! U1 setvar "device.reset_button_enable" "off"

```

**Values**

              - `"on"` the printer reset button is enabled.

              - `"off"` the printer reset button is disabled.

**Default**
```
          "on"

```

**Getvar**


To determine the status of the printer reset button:

```
       ! U1 getvar "device.reset_button_enable"

```

**NOTE:** This setting is not defaulted as part of a factory default ( `^JUF` or `^default` ). The setting
is persistent across a power cycle or rest ( `~JR` or `device.reset` ).


744


SGD Printer Commands
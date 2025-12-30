# power.sleep.cradle



This command enables or disables the sleep timeout feature while the printer is docked in the cradle. In
the ZQ5 printer, the sleep timeout in the cradle is enabled by default. In the ZQ3 and ZQ6 printers, the
sleep timeout in the cradle feature is disabled by default. This is done so as to manage the printer not
having the Wake on BT, Wake on WLAN, and Wake feature on Ethernet support.


The command only affects the sleep timeout in cradle. If the user presses the power button, then the
printer can still go to sleep, regardless of the setting.


**Setvar**


To set the command:

```
       ! U1 setvar "power.sleep.cradle" "values"

```

**Values**

              - `"enabled"` indicates that `power.sleep.timeout` is honored while the printer is docked in a
cradle.

              - `"disabled"` indicates that `power.sleep.timeout` is disabled while the printer is docked in
a cradle.


**Default Value**

`"disabled"` for ZQ3, ZQ6

`"enabled"` for ZQ5


**Getvar**


To return the current setting:

```
       ! U1 getvar "power.sleep.cradle"

```

**Example**


In the example below, the getvar returns the current setting of the sleep timeout in cradle feature.

```
       ! U1 getvar "power.sleep.cradle" "enabled"

```

958


SGD Printer Commands
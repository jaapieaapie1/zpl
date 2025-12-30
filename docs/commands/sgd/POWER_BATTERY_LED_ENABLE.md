# power.battery_led_enable



This command will enable or disable the illumination of the Extended Smart battery LED when one or more
of the `power.battery` thresholds have been reached.


**Setvar**


To enable or disable the illumination of the Extended Smart battery LED whe-n one or more of the
`power.battery` thresholds have been reached:

```
       ! U1 setvar "power.battery_led_enable" "value"

```

**Values**

`"on"` enables the Extended Smart battery LED

`"off"` disables the Extended Smart battery LED

**Default**
```
          "on"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "power.battery_led_enable"

```

942


SGD Printer Commands
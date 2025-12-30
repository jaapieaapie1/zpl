# device.set_clock_to_build_date



Enables or disables a lower bound of the firmware build date for the rtc.date SGD.


If enabled, when the printer powers up and it finds and RTC date earlier than the firmware build date, it will
set the RTC date to the firmware build date.


**Setvar**


To enable or disable a lower bound of the firmware build date for the rtc.date SGD:

```
       ! U1 setvar "device.set_clock_to_build_date" "value"

```

**Values**

              - `"enabled"`

              - `"disabled"`

**Default**
```
          "enabled"

```

**Getvar**

To retrieve the firmware build date for the `rtc.date` SGD:

```
       ! U1 getvar "device.set_clock_to_build_date"

```

766


SGD Printer Commands
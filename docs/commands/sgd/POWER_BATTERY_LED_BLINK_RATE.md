# power.battery_led_blink_rate



Sets the Extended Smart Battery LED blink rate. The rate is set in multiples of 0.5 seconds.


**Setvar**


To set the Extended Smart Battery LED blink rate:

```
       ! U1 setvar "power.battery_led_blink_rate" "value"

```

**Values**

A number from `"0"` to `"127"` . The rate is set in multiples of 0.5 seconds.

**Default**
```
          "2"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "power.battery_led_blink_rate"

```

**Example**


The rate is set in multiples of 0.5 seconds. To achieve an On time of 1 second and an Off time of 4.5
seconds, one would use the following configuration:


On Duration = 2 (2 * 0.5 = 1 second), see power.battery_led_on_duration on page 944


Off Duration = 9 (9 * 0.5 = 4.5 seconds), see power.battery_led_off_duration on page 943


Blink Rate = 2 (2 * 0.5 = 1 second)


941


SGD Printer Commands
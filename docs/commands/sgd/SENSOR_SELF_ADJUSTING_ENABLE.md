# sensor.self_adjusting_enable



Enables the self-adjusting gap sensor.


**Setvar**


To enable the self-adjusting gap sensor:

```
       ! U1 setvar "sensor.self_adjusting_enable" "value"

```

**Values**

              - `"yes"` enables the self-adjusting gap sensor and disables the traditional gap sensor

              - `"no"` disables the self-adjusting gap sensor and enables the traditional gap sensor


**Getvar**


To retrieve the self-adjusting gap sensor:

```
       ! U1 getvar "sensor.self_adjusting_enable"

```

**Example**

This `setvar` example shows the value set to `"yes"` .

```
       ! U1 setvar "sensor.self_adjusting_enable" "yes"

```

The `setvar` value is the `getvar` result. In this example, the `getvar` result is `"yes"` .


1010




SGD Printer Commands
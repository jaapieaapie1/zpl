# ezpl.power_up_action



This command sets what happens to the media when the printer is powered on. This command is similar to
the `^MF` ZPL command.


**Setvar**


To set the media motion and calibration setting at printer power up:

```
       ! U1 setvar "ezpl.power_up_action" "value"

```

**Values**

              - `"calibrate"`

              - `"feed"`

              - `"length"`

              - `"no motion"`

              - `"short cal"`

**Default**
```
          "calibrate"

```

**Getvar**


To return the current power up media motion and calibration settings:

```
       ! U1 getvar "ezpl.power_up_action"

```

**Example**

This `setvar` example sets the power up calibration setting to `"length"` .

```
       ! U1 setvar "ezpl.power_up_action" "length"

```

811


SGD Printer Commands
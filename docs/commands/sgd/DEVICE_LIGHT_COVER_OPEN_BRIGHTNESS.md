# device.light.cover_open_brightness



This command sets the brightness level for the Cover Open light.


**Setvar**


To set the brightness level for the cover open LEDs:

```
       ! U1 setvar "device.light.cover_open_brightness" "value"

```

**Values**

              - `"high"` the LEDs display at maximum brightness when the cover is open

              - `"medium"` the LEDs display at medium brightness when the cover is open

              - `"low"` the LEDs display at lowest brightness when the cover is open

              - `"off"` the LEDs remain off at all times

**Default**
```
          "high"

```

**Getvar**


To retrieve the current brightness level setting for the cover open LEDs:

```
       ! U1 getvar "device.light.cover_open_brightness"

```

**Example**

This `setvar` example shows the value set to `"low"` .

```
       ! U1 setvar "device.light.cover_open_brightness" "low"

```

714


SGD Printer Commands
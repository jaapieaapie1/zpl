# device.light.head_open_brightness



This command sets the brightness level for the Head Open light.


**Setvar**


To set the brightness level for the head open LEDs:

```
       ! U1 setvar "device.light.head_open_brightness" "value"

```

**Values**

              - `"high"` the LEDs display at maximum brightness when the head is open

              - `"medium"` the LEDs display at medium brightness when the head is open

              - `"low"` the LEDs display at lowest brightness when the head is open

              - `"off"` the LEDs remains off at all times

**Default**
```
          "high"

```

**Getvar**


To retrieve the current brightness level setting for the head open LEDs:

```
       ! U1 getvar "device.light.head_open_brightness"

```

**Example**

This `setvar` example shows the value set to `"medium"` .

```
       ! U1 setvar "device.light.head_open_brightness" "medium"

```

715


SGD Printer Commands
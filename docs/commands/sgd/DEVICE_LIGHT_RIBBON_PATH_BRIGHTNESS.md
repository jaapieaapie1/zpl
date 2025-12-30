# device.light.ribbon_path_brightness



Sets the brightness level for the ribbon path LED.


**Setvar**


To set the brightness level for the ribbon path LED:

```
       ! U1 setvar "device.light.ribbon_path_brightness" "value"

```

**Values**

              - `"off"`

              - `"low"`

              - `"medium"`

              - `"high"`

**Default**
```
          "high"

```

**Example**

```
       ! U1 setvar "device.light.ribbon_path_brightness" "low"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "device.light.ribbon_path_brightness"

```

**Result**

```
          "low"

```

791


SGD Printer Commands
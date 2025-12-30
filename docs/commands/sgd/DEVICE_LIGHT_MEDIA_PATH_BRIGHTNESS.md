# device.light.media_path_brightness



Sets the brightness level for the media path LED.


**Setvar**


To set the brightness level for the media path LED:

```
       ! U1 setvar "device.light.media_path_brightness" "value"

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
       ! U1 setvar "device.light.media_path_brightness" "low"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "device.light.media_path_brightness"

```

**Result**

```
          "low"

```

792


SGD Printer Commands
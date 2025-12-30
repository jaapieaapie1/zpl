# display.backlight



This parameter determines if the printer display backlight will be active. Valid only on printers with a display
installed.


**Setvar**


To instruct the printer to turn or off the backlight display:

```
       ! U1 setvar "display.backlight" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**

```
          "on"

```

**Getvar**


To return if the display backlight is on or off:

```
       ! U1 getvar "display.backlight"

```

**Example**

This example sets the backlight display to `"on"` .

```
       ! U1 setvar "display.backlight" "on"

```

794


SGD Printer Commands
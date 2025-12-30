# display.backlight_on_time



This command sets the amount of time before the backlight turns off. Valid only on printers with a display
installed.


**Setvar**


To set the display length in seconds:

```
       ! U1 setvar "display.backlight_on_time" "time"

```

**Values**
```
          0-8191
```

**Default**
```
          0

```

**NOTE:** If the value is set to 0, the backlight will remain on.


**Getvar**


To return the display length in seconds:

```
       ! U1 getvar "display.backlight_on_time"

```

**Example**

This `setvar` example shows the value set to one minute (60 seconds).

```
       ! U1 setvar "display.backlight_on_time" "60"

```

795


SGD Printer Commands
# media.present.loop_length_max



This printer setting determines the maximum allowed length of the presenter loop.


**Setvar**


This command instructs the printer to change the presenter loop length.

```
       ! U1 setvar "media.present.loop_length_max" "value"

```

**Values**

              - `"0"` feeds paper straight through to the presenter

              - `"3"` to `"1023"` specifies the loop length in mm

**Default**
```
          "400"

```

**Getvar**


This command instructs the printer to respond with the currently set presenter loop length.

```
       ! U1 getvar "media.present.loop_length_max"

```

870




SGD Printer Commands
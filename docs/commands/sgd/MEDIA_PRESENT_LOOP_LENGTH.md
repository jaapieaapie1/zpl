# media.present.loop_length



This printer setting determines the length of the presenter loop. If loop_length is greater than
loop_length_max (see `media.present.loop_length_max` ) then it will be set equal to
loop_length_max.


**Setvar**


This command instructs the printer to change the presenter loop length.

```
       ! U1 setvar "media.present.loop_length" "value"

```

**Values**

              - `"0"` means paper is fed straight through the presenter

              - 3-1023 specifies a loop length in mm


**Default**
```
          "400"

```

**Getvar**


This command instructs the printer to respond with the currently set presenter loop length.

```
       ! U1 getvar "media.present.loop_length"

```

869


SGD Printer Commands
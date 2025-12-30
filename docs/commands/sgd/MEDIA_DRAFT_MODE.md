# media.draft_mode



This command puts the printer into draft mode setting.


**NOTE:** Setting the printer to draft mode may result in poorer print quality depending on print
speed, label configurations, etc.


**Setvar**


To set the value:

```
       ! U1 setvar "media.draft_mode" "value"

```

**Values**


Accepted values are different for Link-OS and legacy printers.


Link-OS printers accept:

              - `"on"` indicates faster ramp (acceleration) speed

              - `"off"` indicates normal ramp (acceleration) speed

Legacy printers accept:

              - `"enabled"` indicates faster ramp (acceleration) speed

              - `"disabled"` indicates normal ramp (acceleration) speed

**Default**

Link-OS printers: `"off"`

Legacy printers: `"disabled"`


**Getvar**


To return the currently set value:

```
       ! U1 getvar "media.draft_mode"

```

858


SGD Printer Commands
# ezpl.head_close_action



This command sets what happens to the media after the printhead is closed and the printer is taken out of
pause.

This command is similar to the `^MF` ZPL command.


**Setvar**


To instruct the printer on which action to perform when the printhead is closed:

```
       ! U1 setvar "ezpl.head_close_action" "value"

```

**Values**

              - `"feed"` feed to the first web after sensor

              - `"calibrate"` is used to force a label length measurement and adjust the media and ribbon
sensor values.

              - `"length"` is used to set the label length. Depending on the size of the label, the printer feeds
one or more blank labels.

              - `"no motion"` no media feed

              - `"short cal"` short calibration

**Default**
```
          "calibrate"

```

**Getvar**


To return the current set of action to be performed when the printhead is closed:

```
       ! U1 getvar "ezpl.head_close_action"

```

**Example**

This `setvar` example sets the calibration method to short calibration.

```
       ! U1 setvar "ezpl.head_close_action" "short cal"

```

806


SGD Printer Commands
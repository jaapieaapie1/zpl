# media.present.present_timeout



This printer setting determines how long the printer will wait after a present event to clear the label. See
^KV ZPL command.


**Setvar**


This command instructs the printer to change the presenter function mode.

```
       ! U1 setvar "media.present.present_timeout" "value"

```

**Values**
`"0"` to `"300"`

If label is not taken, retract label when timeout expires. Timeout is in seconds. Zero (0) indicates
that there is no timeout. The label will stay presented until removed manually or a new label is
printed.


**Getvar**


This command instructs the printer to respond with the currently set presenter function mode.

```
       ! U1 getvar "media.present.present_timeout"

```

872


SGD Printer Commands
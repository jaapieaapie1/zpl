# media.present.present_type



This printer setting determines the way that the printer performs a present command. See `^KV` ZPL
command.


**Setvar**


This command instructs the printer to change the presenter function mode.

```
       ! U1 setvar "media.present.present_type" "value"

```

**Values**

              - `"0"` Ejects page when new page is printed

              - `"1"` Retracts page when new page is printed

              - `"2"` Does nothing when new page is printed


**Getvar**


This command instructs the printer to respond with the currently set presenter function mode.

```
       ! U1 getvar "media.present.present_type"

```

873


SGD Printer Commands
# sensor.peeler



Obtains current peeler sensor status.


**Getvar**


To retrieve the current peeler sensor status:

```
       ! U1 getvar "sensor.peeler"

```

**Result**

              - `"clear"` means the last printed item has been removed, or there are no items waiting to be
removed.

              - `"not clear"` means the last printed item has not yet been removed.


1005


SGD Printer Commands
# device.slot_1



This command retrieves the type of board installed in the bottom slot of a ZT400 series printer, or in the
single expansion slot of a ZT200 series printer.


**Getvar**


To retrieve the type of board installed in the bottom slot of a ZT400 series printer:

```
       ! U1 getvar "device.slot_1"

```

**Values**

              - `"empty"` no board installed

              - `"parallel"` a parallel communications board is installed

              - `"wired"` a wired PrintServer board is installed

              - `"wireless"` a wireless PrintServer board is installed


767


SGD Printer Commands
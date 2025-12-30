# device.slot_2



This command retrieves the type of board installed in the bottom slot of a ZT400 series printer.


**Getvar**


To retrieve the type of board installed in the bottom slot:

```
       ! U1 getvar "device.slot_2"

```

**Values**

              - `"empty"` no board installed

              - `"parallel"` a parallel communications board is installed

              - `"wired"` a wired PrintServer board is installed

              - `"wireless"` a wireless PrintServer board is installed


768


SGD Printer Commands
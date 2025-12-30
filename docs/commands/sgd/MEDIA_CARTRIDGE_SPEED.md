# media.cartridge.speed



This command fetches the print speed for the media cartridge.


**Getvar**


To get the print speed for the cartridge:

```
       ! U1 getvar "media.cartridge.speed"

```

**Result**

`"0"` indicates that the cartridge is not installed
```
          "2"
          "4"
```

Currently, the only cartridge speeds supported are `"2"` and `"4"` .


852


SGD Printer Commands
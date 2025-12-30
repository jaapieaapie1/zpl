# media.cartridge.width



This command returns the width of the media cartridge installed in the printer.


**Getvar**


To fetch the width of the media cartridge:

```
       ! U1 getvar "media.cartridge.width"

```

**Result**


A numeric value specified in dots.

`"0"` indicates that the cartridge is not installed

**Example**


In the example below, the getvar returns with the width of the media cartridge in dots.

```
          ! U1 getvar "media.cartridge.width" "300"

```

850




SGD Printer Commands
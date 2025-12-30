# media.cartridge.length



This command returns the initial label length of the media cartridge installed in the printer.


**Getvar**


To fetch the length of the media cartridge:

```
       ! U1 getvar "media.cartridge.length"

```

**Result**


A numeric value specified in dots.

`"0"` indicates that no cartridge is installed

**Example**

In the example below, the `getvar` returns with the width of the media cartridge in dots.

```
          ! U1 getvar "media.cartridge.length" "300"

```

853


SGD Printer Commands
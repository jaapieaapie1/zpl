# media.cartridge.part_number



This command returns the part number of the media cartridge installed in the printer.


**Getvar**


To get the part number of the media cartridge:

```
       ! U1 getvar "media.cartridge.part_number"

```

**Result**


0 to 16 character string

`""` indicates that no cartridge installed

**Example**


In this example, the getvar returns with the part number of the media cartridge.

```
          ! U1 getvar "media.cartridge.part_number" "100127132K"

```

855


SGD Printer Commands
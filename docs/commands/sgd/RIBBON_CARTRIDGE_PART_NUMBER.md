# ribbon.cartridge.part_number



This command retrieves the part number of the ribbon cartridge installed in the printer. There is a 12
character max for the size of string returned since the cartridge allows for 10 character part numbers.


If a ribbon cartridge is not installed, or if the ribbon cartridge option is not present, then the command
returns an empty string.


**Getvar**


To return the part number of the ribbon cartridge:

```
       ! U1 getvar "ribbon.cartridge.part_number"

```

**Result**

`"value"` <= 12 characters

`""` means ribbon cartridge is not installed or not available

**Example**

In this example, the `getvar` returns the part number of the ribbon cartridge.

```
          ! U1 getvar "ribbon.cartridge.part_number" "123456789A"

```

966


SGD Printer Commands
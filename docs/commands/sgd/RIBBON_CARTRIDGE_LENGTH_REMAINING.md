# ribbon.cartridge.length_remaining



This command retrieves the length of ribbon remaining on the cartridge. This is specified in meters.


If a cartridge is not installed, the printer returns "0". If the cartridge option is not available in the printer,
then the printer returns an empty string.


**Getvar**


To return the length of ribbon remaining on the cartridge:

```
       ! U1 getvar "ribbon.cartridge.length_remaining"

```

**Result**

              - `"0"` to `"74"` meters

              - `"0"` means cartridge not installed

              - `""` cartridge is not available


967


SGD Printer Commands
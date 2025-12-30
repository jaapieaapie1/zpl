# ribbon.cartridge.length



This command returns the original length of the ribbon cartridge installed in the printer. This is specified in
meters. If a ribbon cartridge is not installed, then the printer returns "0". If the ribbon cartridge option is not
present, then the command returns an empty string.


**Getvar**


To return the current setting:

```
       ! U1 getvar "ribbon.cartridge.length"

```

**Result**

              - `"0"` indicates the cartridge is not installed

              - `""` indicates the cartridge option is not present


**Example**

In this example, the `getvar` returns the original length of the ribbon cartridge.

```
       ! U1 getvar "ribbon.cartridge.length" "100"

```

968


SGD Printer Commands
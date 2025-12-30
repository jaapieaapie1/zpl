# device.languages



This command identifies the programming language that the printer is currently using.


**Setvar**


To set the printer to the required programming language:

```
       ! U1 setvar "device.languages" "value"

```

**Values**

              - `"epl"` Eltron Programming Language

              - `"epl_zpl"` Eltron Programming Language and Zebra Programming Language

              - `"zpl"` Zebra Programming Language

              - `"hybrid_xml_zpl"` XML and ZPL Programming Languages

              - `"apl-d"` Virtual Device-D (only Link-OS printers)

              - `"apl-t"` Virtual Device-T (only desktop and table top printers with Link-OS)

              - `"apl-e"` Virtual Device-E (only mobile printers with Link-OS)

              - `"apl-o"` Virtual Device-O (only mobile printers with Link-OS)

              - `"apl-i"` Virtual Device-I (only Link-OS printers)


**NOTE:** Not all values are accepted on all printers. Use the `! U1 getvar "allcv"`
command to see the range of values that your printer supports. Values other than those
listed may be available depending on the firmware version being used.


**NOTE:** `"zpl" and "hybrid_xml_zpl"` are equivalent. When the setvar is set to
`"zpl"`, the getvar result will always be `"hybrid_xml_zpl".`

**Default**
```
          "epl_zpl"

```

**Getvar**


To retrieve the programming language that the printer is currently using:

```
       ! U1 getvar "device.languages"

```

**Example**

This `setvar` example sets the programming language to `"hybrid_xml_zpl"` using the shorter value of
`"zpl"` .

```
       ! U1 setvar "device.languages" "zpl"

```

713


SGD Printer Commands
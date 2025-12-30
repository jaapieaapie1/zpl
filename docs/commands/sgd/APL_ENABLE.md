# apl.enable



The `setvar` enables or disables a Virtual Device. The `getvar` returns the currently enabled Virtual
Device.


**Setvar**


To enable or disable a virtual device:

```
       ! U1 setvar "apl.enable" "value"

```

**Values**

`"none"`, `"apl-d"`, `"apl-i"`, `"apl-e"`, `"apl-l"`, `"apl-m"`, `"apl-mi"`, `"apl-o"`, `"apl-t"`,
```
          "pdf"
```

**Default**


NA


**Example**

```
       ! U1 setvar "apl.enable" "pdf"

```

**Getvar**


To return the currently enabled Virtual Device:

```
       ! U1 getvar "apl.enable"

```

**NOTE:** Only the Virtual Device apps loaded on the printer can be enabled.


631


SGD Printer Commands
# zbi.enable



This command enables ZBI on the printer.


**Setvar**


To set the command:

```
       ! U1 setvar "zbi.enable" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "zbi.enable"

```

1057
# display.root_wml



This command specifies which control file is first processed by the front panel of the printer.


**Setvar**


To specify which control file is first processed by the printer’s front panel:

```
       ! U1 setvar "display.root_wml" "value"

```

**Values**


Any file name with a maximum of 128 characters in length.


**Default**

              - `Z:INDEX420.WML` for the QLn420 printers

              - `Z:INDEX320.WML` for the QLn220 and QLn320 printers

              - `Z:INDEX.WML` for all other printers

If the value is `""` on power-up, then `Z:INDEX.WML` is used.


**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "display.root_wml"

```

804


SGD Printer Commands
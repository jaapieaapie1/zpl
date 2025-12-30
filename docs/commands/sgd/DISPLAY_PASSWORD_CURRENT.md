# display.password.current



This command defines the front panel password.


[For more information about the front panel password, refer to the Zebra Link-OS PrintSecure Printer](https://www.zebra.com/content/dam/support-dam/en/documentation/unrestricted/guide/software/printsecure-administration-guide-en.pdf)
[Administration Guide.](https://www.zebra.com/content/dam/support-dam/en/documentation/unrestricted/guide/software/printsecure-administration-guide-en.pdf)


**Setvar**


To set the front panel password:

```
       ! U1 setvar "display.password.current" "value"

```

**Values**

`0000` to `9999` (four digits)

**Default**


              - **For printers purchased in the EMEA region after August 1, 2025:** The default value is empty
and must be set before it can be used on the printer.

              - **For all other printers:** `1234`


**Example**

This `setvar` example shows the front panel password set to `"5723"` .

```
       ! U1 setvar "display.password.current" "5723"

```

802


SGD Printer Commands
# usb.host.fn_field_data



This command is used to collect a user’s response to a presented `^FN` prompt.


**IMPORTANT:** This command is used only in the context of the on-printer Print Station application.
Altering the use of this command in the WML can make the Print Station application nonfunctional. It is recommended changes to portions of this portion of the menu system be done by
Zebra Professional Services team.


**Setvar**

To collect a user’s response to a presented `^FN` prompt:

```
       ! U1 setvar "usb.host.fn_field_data" "value"

```

**Values**


The text entered by the user via a USB Human Interface Device (HID) keyboard.


**Default**
```
          "NULL"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "usb.host.fn_field_data"

```

**Result**


**Result**


One of the following:

          - `"NULL"`

          - The user response to an `^FN` prompt


1021


SGD Printer Commands
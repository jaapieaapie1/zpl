# usb.halt




SGD Printer Commands


This command controls whether the printer allows communication over the USB port when the printer is in
an error condition.


**Setvar**


To control whether the printer allows communication over the USB port when the printer is in an error
condition:

```
! U1 setvar "usb.halt" "value"

```

**Values**

    - `"yes"`

    - `"no"`

**Default**
```
   "no"

```

**Getvar**

To return the current `"usb.halt"` setting stored in the printer:

```
! U1 getvar "usb.halt"

```

**Example**

```
! U1 setvar "usb.halt" "yes"

```

1019


SGD Printer Commands
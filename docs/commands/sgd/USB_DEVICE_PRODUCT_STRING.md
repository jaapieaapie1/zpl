# usb.device.product_string



This command returns the manufacturer-assigned string describing a particular USB product.


**Getvar**


To instruct the printer to respond with the product string description:

```
       ! U1 getvar "usb.device.product_string"

```

**Example**


Issuing the command on a ZT210 printer:

```
       ! U1 getvar "usb.device.product_string"
       "ZT210"

```

Issuing the command on a QLn320 printer:

```
       ! U1 getvar "usb.device.product_string"
       "ZTC QLn320-203dpi CPCL"

```

**NOTE:** For firmware V68.19.0 and V72.19.0, the return value was changed to the longer name,
which is the same as the USB PID. The QLnXXX is replaced by the printer model and number for
each printer.


1016


SGD Printer Commands
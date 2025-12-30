# usb.device.product_id



This command retrieves the Product Identification number that a manufacturer assigned to a particular
product. This number, along with the Vendor ID, allows a USB host to distinguish between devices.


**Getvar**


To retrieve the product ID:

```
       ! U1 getvar "usb.device.product_id"

```

**Example**

```
       ! U1 getvar "usb.device.product_id"
       "003D"

```

1015


SGD Printer Commands
# usb.device.vendor_id



This command returns the Vendor Identification number that the USB-IF organization has assigned to a
manufacturer. This number, along with the Product ID, allows a USB host to distinguish between devices.


**Getvar**


To retrieve the vendor ID of the device:

```
       ! U1 getvar "usb.device.vendor_id"

```

**Example**

```
       ! U1 getvar "usb.device.vendor_id"
       "0a5f"

```

1018
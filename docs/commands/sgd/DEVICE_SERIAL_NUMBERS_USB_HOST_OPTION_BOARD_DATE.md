# device.serial_numbers.usb_host_option_board_date



This command retrieves the USB Host option board date.


For printers that do not store this value, the printer returns an empty string.


**Getvar**


To return the USB host option board date:

```
       ! U1 getvar "device.serial_numbers.usb_host_option_board_date"

```

**Example**

In this example, the `getvar` returns the manufacturing date of the USB host option board.

```
       ! U1 getvar "device.serial_numbers.usb_host_option_board_date" "12/31/2014"

```

762


SGD Printer Commands
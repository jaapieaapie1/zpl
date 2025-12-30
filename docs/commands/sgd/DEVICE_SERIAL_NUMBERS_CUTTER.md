# device.serial_numbers.cutter



This command returns the serial number of the cutter, if installed.


For printers that do not store this value, the printer returns an empty string.


**Getvar**


To return the serial number of the cutter board:

```
       ! U1 getvar "device.serial_numbers.cutter"

```

**Value**


A hexadecimal representation of the control panel serial number.


**Example**

In this example, the `getvar` returns the serial number of the cutter board.

```
       ! U1 getvar "device.serial_numbers.cutter"

       "0123456789ABCDEF"

```

758


SGD Printer Commands
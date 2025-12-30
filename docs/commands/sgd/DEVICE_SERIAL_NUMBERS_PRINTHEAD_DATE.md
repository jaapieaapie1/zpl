# device.serial_numbers.printhead_date



This command retrieves the printhead date. For printers that do not store this value, the printer returns an
empty string.


**Getvar**


To return the printhead date:

```
       ! U1 getvar "device.serial_numbers.printhead_date"

```

**Example**

In this example, the `getvar` returns the manufacturing date of the printhead.

```
       ! U1 getvar "device.serial_numbers.printhead_date" "12/31/2014"

```

761


SGD Printer Commands
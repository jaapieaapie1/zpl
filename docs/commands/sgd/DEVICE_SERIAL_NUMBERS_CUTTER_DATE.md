# device.serial_numbers.cutter_date



This command returns the cutter date. For printers that do not store this value, the printer returns an empty
string.


**Getvar**


To return the manufacturing date of the cutter board:

```
       ! U1 getvar "device.serial_numbers.cutter_date"

```

**Example**


In this example, the getvar returns the manufacturing date of the cutter board.

```
       ! U1 getvar "device.serial_numbers.cutter_date"

       "12/31/2014"

```

759


SGD Printer Commands
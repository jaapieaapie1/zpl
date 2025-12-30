# device.serial_numbers.parallel_option_board_date



This command retrieves the parallel port option board date.


For printers that do not store this value, the printer returns an empty string.


**Getvar**


To return the parallel option board date:

```
       ! U1 getvar "device.serial_numbers.parallel_option_board_date"

```

**Example**

In this example, the `getvar` returns the manufacturing date of the parallel port option board.

```
       ! U1 getvar "device.serial_numbers.parallel_option_board_date"
       "12/31/2014"

```

765


SGD Printer Commands
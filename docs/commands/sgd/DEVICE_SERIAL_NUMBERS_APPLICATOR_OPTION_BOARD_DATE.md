# device.serial_numbers.applicator_option_board_date



This command retrieves the applicator option board date.


For printers that do not store this value, the printer returns an empty string.


**Getvar**


To return the current setting:

```
       ! U1 getvar "device.serial_numbers.applicator_option_board_date"

```

**Example**


In this example, the printer reports the application option boards date.

```
       ! U1 getvar "device.serial_numbers.applicator_option_board_date"
       "12/31/2014"

```

754


SGD Printer Commands
# device.serial_number.option_board_date



Returns the date the option board was made.


**NOTE:** This command is functional only on printers that had their option board manufacturing
date programmed when they were created. Older printers that do not have the option board
creation date programmed will return a `"?"` or empty string.


**Getvar**


To return the date the option board was made:

```
       ! U1 getvar "device.serial_number.option_board_date"

```

**Result**


The date in mm/dd/yyyy format.


750




SGD Printer Commands
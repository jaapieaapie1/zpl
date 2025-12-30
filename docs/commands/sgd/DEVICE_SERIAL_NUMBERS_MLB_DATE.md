# device.serial_numbers.mlb_date



Returns the date the main logic board (MLB) was made.


**NOTE:** This command is functional only on printers that had their MLB manufacturing date
programmed when they were created. Older printers that do not have the MLB creation date
programmed will return a `"?"` or empty string.


**Getvar**


To return the date the main logic board (MLB) was made:

```
       ! U1 getvar "device.serial_numbers.mlb_date"

```

**Result**


The date in mm/dd/yyyy format.


752


SGD Printer Commands
# device.serial_numbers.control_panel_date



This command returns the date the control panel was made.


**NOTE:** This command is functional only on printers that had their control panel manufacturing
date programmed when they were created. Older printers that do not have the control panel
creation date programmed will return a `"?"` or empty string.


**Getvar**


To return the date the control panel was made:

```
       ! U1 getvar "device.serial_numbers.control_panel_date"

```

**Result**


The date in mm/dd/yyyy format.


751


SGD Printer Commands
# device.printhead.test.detail



This command returns the results of the last printhead test for the resistance values. This command is not
reported in the ALLCV.


**Getvar**


To get the summary of the printer’s printhead test details:

```
       ! U1 getvar "device.printhead.test.detail"

```

**Result**


A comma separated string as given below. Although the content below is shown on individual lines,
it is displayed as one line of comma separated values.


              - Current Date (as reported by rtc.date),


              - Current Time (as reported by rtc.time),

              - Odometer Value in cm (as reported by `"odometer.total_print_length"` ),

              - Part Number of the Printhead,


              - Serial Number of the Printhead,


              - Resistance profile of each Printhead element


**NOTE:** The command will return a response for all Link-OS printers. However only
printers that support the head test will display valid values. For all unsupported printers,
the result is "Test Not Run".


736


SGD Printer Commands
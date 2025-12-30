# device.printhead.test.summary



This command retrieves a summary of the printer’s printhead test results. This command mimics the results
of the `~HQJT` ZPL command output.


**Getvar**


To get the summary of the printer’s printhead test results:

```
       ! U1 getvar "device.printhead.test.summary"

```

**Result**


A string in the format of A, B, C, D, E.


              - A number Element Failure


              - B Manual (M) or automatic (A) range


              - C first test element


              - D last test element


              - E failure count


**NOTE:** The command will return a response for all LOS printers. However only printers
that support the head test will display valid values. For all unsupported printers, C and D
above will always be 0.


734


SGD Printer Commands
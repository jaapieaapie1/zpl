# device.feature.head_element_test



This command retrieves the head element test status on the printer.


**Getvar**


To return the head element test feature availability:

```
       ! U1 getvar "device.feature.head_element_test"

```

**Result**

              - `"present"` if head test is present on the printer

              - `"not present"` if head test is present on the printer

              - `"not available"` if head test is not available on the platform


695


SGD Printer Commands
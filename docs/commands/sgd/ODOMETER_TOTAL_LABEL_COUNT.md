# odometer.total_label_count



This command returns the total number of labels printed over the life of the printer.


**NOTE:** The number returned does not include form feeds or calibration labels.


**Getvar**


To return the current setting value:

```
       ! U1 getvar "odometer.total_label_count"

```

**Example**


To get the total number of labels printed to date:

```
       ! U1 getvar "odometer.total_label_count"
       (sample) "31084"

```

933


SGD Printer Commands
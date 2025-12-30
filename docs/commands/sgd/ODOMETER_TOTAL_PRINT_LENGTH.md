# odometer.total_print_length



This command returns the total length of all media movement over the life of the printer.


**NOTE:** The number returned includes all media movement including backfeeds.


**Getvar**


To return the current setting value:

```
       ! U1 getvar "odometer.total_print_length"

```

**Default**
```
          "0"

```

**Example**


To get the total length of media printed to date:

```
       ! U1 getvar "odometer.total_print_length"
       (sample) "8560 INCHES, 21744 CENTIMETERS"

```

932


SGD Printer Commands
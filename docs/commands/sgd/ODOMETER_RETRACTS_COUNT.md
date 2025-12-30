# odometer.retracts_count



This printer value records the number of times a label has been retracted since the last time the counter
has been reset.


**Setvar**


To reset the current count of retractions:

```
       ! U1 setvar "odometer.retracts_count" "value"

```

**Values**

`"0"` resets the counter

**Default**
NA


**Getvar**


To respond with the current number of retractions that have happened since the last time the counter was
reset:

```
       ! U1 getvar "odometer.retracts_count"

```

926


SGD Printer Commands
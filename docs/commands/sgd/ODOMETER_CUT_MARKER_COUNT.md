# odometer.cut_marker_count



Returns the number of cuts incurred by the cutter or resets the counter to "0". This command tracks the
same events as `odometer.total_cuts`, which cannot be reset.


**Setvar**


To set the number of cuts incurred by the cutter:

```
       ! U1 setvar "odometer.cut_marker_count" "value"

```

**Values**

`"0"` resets the counter to 0


**Getvar**


To return the current setting value:

```
       ! U1 getvar "odometer.cut_marker_count"

```

**Result**

An integer value of `"0"` or greater.


917


SGD Printer Commands
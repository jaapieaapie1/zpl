# power.cycle_count



This command returns the number of charge cycles the battery has performed. A cycle is defined as a
discharge of 80% of the pack's full charge capacity plus the concatenated partial charges that add to 80%
of the pack's full charge capacity.


**Getvar**


To return the number of charge cycles:

```
       ! U1 getvar "power.cycle_count"

```

**Example**

In the example below, the `getvar` returns the number of charge cycles the battery has performed.

```
       ! U1 getvar "power.cycle_count" "77"

```

960




SGD Printer Commands
# odometer.user_total_cuts



This command sets the number of cuts incurred by the cutter. This is the resettable version of the
odometer.total_cuts SGD.


**Setvar**


To set the number of cuts incurred by the cutter:

```
       ! U1 setvar "odometer.user_total_cuts" "0"

```

**Values**

`"0"` resets the cut counter.


**Getvar**


To return the current number of cuts since the last time the cut counter was reset:

```
       ! U1 getvar "odometer.user_total_cuts"

```

**Result**

`"0"` to `"n"`

Here `"n"` is an integer


935


SGD Printer Commands
# odometer.user_label_count



Returns the number of labels printed since the last odometer set command.


**Setvar**


To set the user label count:

```
       ! U1 setvar "odometer.user_label_count" "value"

```

**Values**

`"0"` to `"65000"`


**Related ZPL Commands**

`~RO` 1


**Getvar**


To return the current setting value:

```
       ! U1 getvar "odometer.user_label_count"

```

**Example**


To get the total number of labels printed to date:

```
       ! U1 getvar "odometer.user_label_count"
       (sample) "7544"

```

934


SGD Printer Commands
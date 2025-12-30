# odometer.user_label_count[1|2]



Returns the number of labels printed since the last reset of each resettable odometer.


**Setvar**


To resets the counter value to 0:

```
       ! U1 setvar "odometer.user_label_count1" "value"
       ! U1 setvar "odometer.user_label_count2" "value"

```

**Values**

`"0"` to `"4294967295"`


**Related ZPL Commands**
```
       ~RO

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "odometer.user_label_count1"
       ! U1 getvar "odometer.user_label_count2"

```

**Example**


To get the total number of labels printed on to date:

```
       ! U1 getvar "odometer.user_label_count1"
       "164"

```

936


SGD Printer Commands
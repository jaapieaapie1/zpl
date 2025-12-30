# power.remaining_capacity



This command returns the remaining capacity of the battery in milliamp hours (mAh).


**Getvar**


To return the remaining battery capacity:

```
       ! U1 getvar "power.remaining_capacity"

```

**Result**

`"0 mAh"` to `"65535 mAh"`


**Example**

In the example below, the `getvar` returns the remaining battery capacity of "1846 mAh".

```
       ! U1 getvar "power.remaining_capacity" "1846 mAh"

```

959


SGD Printer Commands
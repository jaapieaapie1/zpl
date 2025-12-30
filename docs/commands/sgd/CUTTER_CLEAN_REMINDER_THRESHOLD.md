# cutter.clean_reminder_threshold



This command sets the threshold to trigger a cutter cleaning reminder alert.


**Setvar**


To set the cutter cleaning reminder feature:

```
       ! U1 setvar "cutter.clean_reminder_threshold" "value"

```

**Values**


0–4294967295


**Default**


100000


**Getvar**

To retrieve the status of the `cutter.clean_reminder_threshold` command:

```
       ! U1 getvar "cutter.clean_reminder_threshold"

```

**Example**

In this example, the `setvar` sets the value to `"200000"` (enabled).

```
       ! U1 setvar "cutter.clean_reminder_threshold" "200000"

```

661


SGD Printer Commands
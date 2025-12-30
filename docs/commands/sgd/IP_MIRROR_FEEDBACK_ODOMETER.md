# ip.mirror.feedback.odometer



This command instructs the printer to set the mirror feedback odometer.


**Setvar**


To set the odometer counter:

```
       ! U1 setvar "ip.mirror.feedback.odometer" "values"

```

**Values**

A numeric value between `"0"` and `"4294967295"` .

**Default**
```
          "0"

```

**Getvar**


To retrieve the mirror feedback odometer value:

```
       ! U1 getvar "ip.mirror.feedback.odometer"

```

**Example**

This `setvar` example shows the value set to `"0"` .

```
       ! U1 setvar "ip.mirror.feedback.odometer" "0"

```

When the `setvar` value is set to `"0"`, the `getvar` result is `"0"` .


1273


SGD Network Commands
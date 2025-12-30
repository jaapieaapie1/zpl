# zpl.label_orientation



This command instructs the printer to rotate the entire label output by 0, 90, 180, or 270 degrees.


**Setvar**


**NOTE:** If this command is set to 180, `print.invert_label` is enabled, and `^POI` is used. The
outcome results in an inverted label orientation (because it is triple inverting).


To assign the value of the label orientation:

```
       ! U1 setvar "zpl.label_orientation" "value"

```

**Values**

`"0"`, `"90"`, `"180"`, `"270"`

**Default**
```
          "0"

```

**Getvar**


To retrieve the value of the label orientation:

```
       ! U1 getvar "zpl.label_orientation"

```

**Example**

This `setvar` example shows the value set to `"90"` .

```
       ! U1 setvar "zpl.label_orientation" "90"

```

1076


SGD Printer Commands
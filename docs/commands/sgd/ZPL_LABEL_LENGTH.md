# zpl.label_length



This command defines the length of the label. This command is necessary when using continuous media
(media that is not divided into separate labels by gaps, spaces, notches, slots, or holes).

This command is equivalent to the `^LL` command.


**Setvar**


To set the label length:

```
       ! U1 setvar "zpl.label_length" "value"

```

**Values**

`"1"` to `"32000"`, (in dots) not to exceed the maximum label length.

While the printer accepts any value for this parameter, the amount of memory installed determines
the maximum length of the label.


**Getvar**


To return the current label length setting:

```
       ! U1 getvar "zpl.label_length"

```

**Comments**


These formulas can be used to determine the value of y:


For 6 dot/mm printheads... Label length in inches x 152.4 (dots/inch) = y

For 8 dot/mm printheads... Label length in inches x 203.2 (dots/inch) = y

For 12 dot/mm printheads... Label length in inches x 304.8 (dots/inch) = y

For 24 dot/mm printheads... Label length in inches x 609.6 (dots/inch) = y


Values for `y` depend on the memory size. If the entered value for `y` exceeds the acceptable limits, the
bottom of the label is cut off. The label also shifts down from top to bottom.


**Example**

```
       ! U1 setvar "zpl.label_length" "1281"

```

1074


SGD Printer Commands
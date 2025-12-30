# usb.mirror.feedback.odometer



This command instructs the printer to set or retrieve the `usb.mirror.feedback.odometer` value.


**Setvar**

To set the `usb.mirror.feedback.odometer` value:

```
       ! U1 setvar "usb.mirror.feedback.odometer" "value"

```

**Values**

`"0"` to `"65535"`

**Default**
```
          "0"

```

**Example**


This example sets the counter to 0, which resets the counter.

```
       ! U1 setvar "usb.mirror.feedback.odometer" "0"

```

**Getvar**

To retrieve the `usb.mirror.feedback.odometer` value:

```
       ! U1 getvar "usb.mirror.feedback.odometer"

```

1037


SGD Printer Commands
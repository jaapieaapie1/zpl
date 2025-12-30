# device.applicator.error_on_pause



Sets whether device applicator errors will be displayed.


**Setvar**


To set whether device applicator errors will be displayed:

```
       ! U1 setvar "device.applicator.error_on_pause" "value"

```

**Values**

`"enabled"` device applicator errors will be displayed, and SERVICE REQUIRED will be asserted.

`"disabled"` device applicator errors will not be displayed.

**Default**
```
          "enabled"
```

**Example**

```
          ! U1 setvar "device.applicator.error_on_pause" "enabled"

```

**Getvar**


To return the current setting value:

```
       ! U1 setvar "device.applicator.error_on_pause" "enabled"

```

778


SGD Printer Commands
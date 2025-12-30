# media.small_label.enhanced_length_calibration



This command sets or retrieves the setting for a feature that enhances the calibration process for small
labels by sampling a longer portion of the media.


**NOTE:** Enabling this feature increases label usage for the calibration process used to determine
label length.


**Setvar**


To enable or disable the enhanced label length calibration feature:

```
       ! U1 setvar "media.small_label.enhanced_length_calibration" "value"

```

**Values**

              - `"enabled"` enables IPP

              - `"disabled"` disables IPP

**Default**
```
          "disabled"

```

**Getvar**


To retrieve the current setting of the enhanced label length calibration feature:

```
       ! U1 getvar "media.small_label.enhanced_length_calibration"

```

**Example**

This `setvar` example shows the value set to `"enabled"` .

```
       ! U1 setvar "media.small_label.enhanced_length_calibration" "enabled"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"enabled"` .


876


SGD Printer Commands
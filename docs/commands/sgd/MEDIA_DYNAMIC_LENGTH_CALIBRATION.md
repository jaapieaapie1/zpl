# media.dynamic_length_calibration



This command enables or disables the dynamic length calibration. This is identical to the first parameter of
the `^XS` command - Dynamic Length Calibration.


**Setvar**


To enable or disable the dynamic length calibration:

```
       U1 setvar "media.dynamic_length_calibration" "value"

```

**Values**

`"on"` indicates dynamic length calibration is enabled.

`"off"` dynamic length calibration is disabled.

**Default Value**

              - `"on"` for Desktop printers

              - `"off"` for Industrial printers


**Getvar**


To return the current setting:

```
       ! U1 getvar "media.dynamic_length_calibration"

```

**Example**
In this example, the getvar returns the current setting of the dynamic length calibration.

```
          ! U1 setvar "media.dynamic_length_calibration" "on"

```

859


SGD Printer Commands
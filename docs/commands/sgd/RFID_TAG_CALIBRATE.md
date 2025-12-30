# rfid.tag.calibrate



Use this command to initiate tag calibration for RFID media. During the process, the printer moves the
media, calibrates the RFID tag position, and determines the optimal settings for the RFID media being
used. Depending on the printer, these settings include the programming position, the antenna element to
use, and the read/write power level to use.


**NOTE:** For more information about RFID tag calibration, refer to the RFID Programming Guide for
[your printer. A copy is available online at www.zebra.com/manuals.](http://www.zebra.com/manuals)


**Setvar**


To initiate tag calibration for RFID media:

```
       ! U1 setvar "rfid.tag.calibrate" "value"

```

**Values**
```
          "restore"
          "run"

```

**Example**

This `setvar` example restores the programming position back to the printer’s default value.

```
       ! U1 setvar "rfid.tag.calibrate" "restore"

```

This `setvar` example performs RFID tag calibration.

```
       ! U1 setvar "rfid.tag.calibrate" "run"

```

1529


SGD RFID Commands
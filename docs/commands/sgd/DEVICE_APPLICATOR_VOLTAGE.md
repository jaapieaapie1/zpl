# device.applicator.voltage



Sets the output voltage of the applicator board. The value will not take effect until a reboot.


**Setvar**


To set the output voltage of the applicator board:

```
       ! U1 setvar "device.applicator.voltage" "value"

```

**Values**

              - `"0"` indicates off

              - `"5"` indicates 5V

              - `"24"` indicates 24V

**Default**

```
          "disabled"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "device.applicator.voltage"

```

**Result**

```
          "5"

```

**Example**

In the setvar example below, the output voltage of the applicator board is set to `"5"` .

```
       ! U1 setvar "device.applicator.voltage" "5"

```

780




SGD Printer Commands
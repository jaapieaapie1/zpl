# rfid.reader_1.power.single_power



This command sets the RFID reader power level for reading and writing to RFID tags for readers with a
single power level.


**NOTE:** This command applies only to the R110Xi HF printer, firmware version R65.X.


**Setvar**


To set the power level for reading and writing:

```
       ! U1 setvar "rfid.reader_1.power.single_power" "value"

```

**Values**

              - `"high"`

              - `"medium"`

              - `"low"`

**Default**
```
          "low"

```

**Getvar**


To respond with the current power level:

```
       ! U1 getvar "rfid.reader_1.power.single_power"

```

This `setvar` example sets the antenna to high power for writing to RFID tags.


**Example**

```
       ! U1 setvar "rfid.reader_1.power.single_power" "high"

```

When the `setvar` value is set to `"high"`, the `getvar` result is `"high"` .


1523


SGD RFID Commands
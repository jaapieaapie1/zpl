# rfid.reader_1.power.write



Use this command to set the RFID write power levels if the desired levels are not achieved through RFID
tag calibration. If not enough power is applied, the tag may not have sufficient power for programming, and
tag data will fail to encode. If too much power is applied, the extra power may cause data communication
errors or may cause the wrong tag to be programmed.


**NOTE:** Printers automatically select the best antenna element and read/write power levels for the
media during RFID transponder calibration. The ZT400 and ZT600 series printers also may set
the levels during an adaptive antenna sweep.


**NOTE:** This parameter is ignored on the R110Xi HF printer because read and write powers cannot
be specified separately. See to set the power level for the R110Xi HF printer.


**Setvar**


Instructs the printer to set the antenna’s write power level.

```
       ! U1 setvar "rfid.reader_1.power.write" "value"

```

**Values**


**RP4T (all firmware versions), R53.16.4Z, V53.17.7, V74.19.6Z, and all Link-OS printers**

              - `"0"` to `"30"`

              - `"up"` Increase the current value by 1

              - `"down"` Decrease the current value by 1

**R53.16.3Z**

`"0"` to `"30"`

**R60.16.x, R62.16.x, R63.16.x, R65.16.x, SP994Q, SP999G, SP1027G, SP1056F, SP1082G, and later**

              - `"0"` to `"30"`

              - `"high"`

              - `"medium"`

              - `"low"`

**Older firmware**

              - `"high"`

              - `"medium"`

              - `"low"`


**Default**


**RP4T (all firmware versions), R53.16.4Z, V53.17.7, V74.19.6Z, and all Link-OS printers**
```
          "16"
```

**R53.16.3Z**
```
          "16"
```

**R60.16.x, R62.16.x, R63.16.x, R65.16.x, SP994Q, SP999G, SP1027G, SP1056F, SP1082G, and later**
```
          "low"

```

1521


SGD RFID Commands


**Older firmware**
```
   "low"

```

**Getvar**


To return the antenna’s write power level:

```
! U1 getvar "rfid.reader_1.power.write"

```

**Example**

This `setvar` example sets the antenna to power setting 16 for writing RFID tags.

```
! U1 setvar "rfid.reader_1.power.write" "16"

```

When the `setvar` value is set to `"16"`, the `getvar` result is `"16"` .


1522


SGD RFID Commands
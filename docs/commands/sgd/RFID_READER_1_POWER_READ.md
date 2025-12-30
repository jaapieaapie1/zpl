# rfid.reader_1.power.read



This command sets the RFID reader power level for reading RFID tags.


**NOTE:** NOTE: Printers automatically select the best antenna element and read/write power levels
for the media during RFID transponder calibration. The ZT400 and ZT600 series printers also
may set the levels during an adaptive antenna sweep. Use ^HL or ~HL on page 412 to view the
antenna element and power settings being used.


**Setvar**


Instructs the printer to set the read power level of the antenna.

```
       ! U1 setvar "rfid.reader_1.power.read" "value"

```

**Values**


**RP4T (all firmware versions), R53.16.4Z, V53.17.7, V74.19.6Z, and all Link-OS printers:**

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

**Older firmware**
```
          "low"

```

**Getvar**


To return the antenna’s read power level:


1519


SGD RFID Commands

```
! U1 getvar "rfid.reader_1.power.read"

```

**Example**

This `setvar` example sets the antenna to power setting 16 for reading RFID tags.

```
! U1 setvar "rfid.reader_1.power.read" "16"

```

When the `setvar` value is set to `"16"`, the `getvar` result is `16` .


1520




SGD RFID Commands
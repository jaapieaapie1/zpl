# media.printmode



This printer setting determines the action the printer takes after a label or group of labels has printed.


**Setvar**


This command is equivalent to ^MM on page 305 .


This command instructs the printer to change the media print mode.

```
       ! U1 setvar "media.printmode" "value"

```

**Values**

              - `"T"` Tear-off [1]

              - `"P"` Peel-off (not available on S-300) [1, 2]

              - `"R"` Rewind (depends on printer model)

              - `"A"` Applicator (depends on printer model) [1]

              - `"C"` (depends on printer model) Cutter [2]

              - `"D"` Delayed cutter [1, 2]

              - `"F"` RFID [1, 2]

              - `"L"` Linerless Peel [2, 3]

              - `"U"` Linerless Rewind [2, 3]

              - `"K"` Kiosk [4]

              - `"V"` Linerless Tear [2]

              - `"S"` Stream [5]

1. Not supported on the KR403 printer.


2. Not supported on the ZE500 printer.


3. Not supported on the ZM400/ZM600 and RZ400/RZ600 printers.


4. Only supported on the KR403 printer.


5. Only supported on the ZE500 printer.


**Getvar**


This command instructs the printer to respond with the the currently set media print mode.

```
       ! U1 getvar "media.printmode"

```

**Example**


This `setvar` example shows the value set to `"T"` .


874


SGD Printer Commands

```
! U1 setvar "media.printmode" "T"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"tear off"` .


**Setvar / Getvar Relation**

|When setvar is|getvar response and control panel display|
|---|---|
|`"T"`|`TEAR OFF`|
|`"P"`|`PEEL OFF`|
|`"R"`|`REWIND`|
|`"A"`|`APPLICATOR`|
|`"C"`|`CUTTER`|
|`"D"`|`DELAYED CUT`|
|`"L"`|`RESERVED`|
|`"U"`|`RESERVED`|
|`"K"`|`KIOSK`|



875


SGD Printer Commands
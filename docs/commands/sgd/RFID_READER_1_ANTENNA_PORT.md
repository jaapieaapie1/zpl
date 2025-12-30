# rfid.reader_1.antenna_port



This command specifies the RFID antenna to be used for RFID operation.


**NOTE:**


          - This applies only to ZT400 and ZT600 series RFID printers, which have multiple antenna
elements. Other printers, which only have one antenna element, always use an antenna
element value of `A1` .

          - Printers automatically select the best antenna element and read/write power levels for the
media during RFID transponder calibration. The ZT400 and ZT600 series printers also may
set the levels during an adaptive antenna sweep. Use ^HL or ~HL on page 412 to view the
antenna element and power settings being used.


**Setvar**


Sets the antenna port.

```
       ! U1 setvar "rfid.reader_1.antenna_port" "value"

```

**Values (ZT400 and ZT600 Series only)**

```
           E1 E2 E3 E4

           D1 D2 D3 D4

           C1 C2 C3 C4

           B1 B2 B3 B4

           A1 A2 A3 A4

```

1517


SGD RFID Commands


**Values (R110Xi4 only)**


NA `F2` `F3` `F4`


NA `E2` `E3` `E4`


NA `D2` `D3` `D4`

```
   C1 C2 C3 C4

   A1 A2 A3 A4

```

**Default**
```
   "A4"

```

**Getvar**


Retrieves the current antenna port.

```
! U1 getvar "rfid.reader_1.antenna_port"

```

**Example**

This `setvar` example shows the selection of antenna port D3.

```
! U1 setvar "rfid.reader_1.antenna_port" "D3"

```

When the `setvar` value is set to `"D3"`, the `getvar` result is `"D3"` .


1518


SGD RFID Commands
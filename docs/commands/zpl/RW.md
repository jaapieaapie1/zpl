# ^RW




ZPL RFID Commands


Use this command to set the RFID read and write power levels if the desired levels are not achieved
through RFID tag calibration. If not enough power is applied, the tag may not have sufficient power for
programming, and tag data will fail to encode. If too much power is applied, the extra power may cause
data communication errors or may cause the wrong tag to be programmed.


**Set RF Power Levels for Read and Write**


**NOTE:** Printers automatically select the best antenna element and read/write power levels for the
media during RFID transponder calibration. The R110Xi4, ZT400 series, and ZT600 series printers
also may set the levels during an adaptive antenna sweep. Use ^HL or ~HL on page 412 to view
the antenna element and power settings being used.


**NOTE:** For Japan, the printer’s maximum RFID read and write power are limited to comply with
local radio regulations. Any power setting of 24 or higher results in the same output.


**Format:** ^RWr,w,a

|Parameters|Details|
|---|---|
|`r `= read power|This parameter sets the power level to match the desired output as<br>calibrated in the factory.<br>**R53.16.3, V53.17.5, and later:**<br>**Values:**0 to 30<br>**Default:**16<br>**R60.16.4, R62.16.4, R63.16.4, SP994Q, SP999G, SP1027G, SP1056F,**<br>**SP1082G, and later:**<br>**Values:**`0` to`30`, `H` (high),`M` (medium),`L` (low)<br>**Default:**`L`<br>**R65.X and older versions of other firmware:**<br>**Values:**<br>`H =`high<br>`M =`medium<br>`L =`low<br>**Default:**`L`|



439


ZPL RFID Commands





|Parameters|Details|
|---|---|
|`w` = write power|**NOTE:** This parameter is ignored on the R110Xi HF printer<br>(firmware version R65.X) because read and write powers cannot<br>be specified separately. The printer uses the value that you<br>specified for read power for both the read and write power<br>settings.<br>This parameter sets the power level to match the desired output as<br>calibrated in the factory.<br>**R53.16.3, V53.17.5, and later:**<br>**Values:**0 to 30<br>**Default:**16<br>**R60.16.4, R62.16.4, R63.16.4, SP994Q, SP999G, SP1027G, SP1056F,**<br>**SP1082G, and later:**<br>**Values:**`0` to`30`, `H` (high),`M` (medium),`L` (low)<br>**Default:**`L`<br>**Older versions of firmware:**<br>**Values:**<br>`H =`high<br>`M =`medium<br>`L =`low<br>**Default:**`L`|


440




ZPL RFID Commands





|Parameters|Details|
|---|---|
|`a` = RFID antenna element<br>selection|**ZD500R, ZQ511/ZQ521, and ZQ630:**<br>This printer only has one antenna element, so the value used is always`A1`.<br>**ZT400 and ZT600:**<br>This parameter specifies the RFID antenna to be used for RFID operation.<br>`E1`, `E2`, `E3`, `E4`<br>`D1`, `D2`, `D3`, `D4`<br>`C1`, `C2`, `C3`, `C4`<br>`B1`, `B2`, `B3`, `B4`<br>`A1`, `A2`, `A3`, `A4`<br>`A4`<br>**(Continued on next page)**|


441


ZPL RFID Commands






|Parameters|Details|
|---|---|
|`a` = RFID antenna element<br>selection|**(Continued from previous page)**<br>**R110Xi4 (V53.17.5 and later):**<br>This parameter specifies the RFID antenna to be used for RFID operation.<br>**Values:**<br>A1, A2, A3, A4, B1, B2, B3, B4, C1, C2, C3, C4, D2, D3, D4, E2, E3, E4, F2, F3,<br>F4 (combinations D1, E1, and F1 are invalid)<br>**Default:**`A4`<br>**R110Xi HF (R65.X):**<br>This parameter selects the antenna port that provides the best results for<br>reading and writing.<br>**Values:**<br>`1 =`antenna port 1<br>`2 =`antenna port 2<br>**Default:**`1`|



**Example:** The following command selects the antenna at row D, column 3 on an R110Xi4 printer:

```
^RW,,D3

```

**Example:** The following command sets the read/write power level to Medium and selects antenna 2 on an
R110Xi HF printer:

```
^RWM,,2

```

**Example:** The following command sets the read and write power levels to High on an R110PAX4 printer:

```
^RWH,H

```

442
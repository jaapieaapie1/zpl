# ~WQ




ZPL Commands


The `~WQ` command triggers the printer to print a label with odometer, maintenance or alert, and printhead
history information.


**Write Query**

**Format:** `~WQquery-type`


|Parameter|Details|
|---|---|
|`query-type`|For detailed examples of these parameters, see~WQ Examples.<br>**Values:**<br>`ES =` requests the printer’s status. For details see,Table 10    Error Flags (~WQES)<br>on page 362 andTable 11    Warning Flags (~WQES) on page 363.<br>`HA =` hardware address of the internal wired print server<br>`JT =` requests a summary of the printer’s printhead test results<br>`MA =` maintenance alert settings<br>`MI =` maintenance information<br>`OD =` odometer<br>`PH =` printhead life history<br>`PP =` printer’s Plug and Play string<br>`SN =` printer’s serial number<br>`UI =` printer’s USB product ID and BCD release version<br>**Default:** must be an accepted value or the command is ignored|



**Table 10** Error Flags (~WQES)

























|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles**<br>**16-9**|**Nibble8**|** Nibble7**|** Nibble6**|**  Nibble5**|**  Nibble4**|**   Nibble3**|**   Nibble2**|**    Nibble1**|
|No Error|0|00000000|0|0|0|0|0|0|0|0|
|Error Present|1|00000000|X|X|X|X|X|X|X|X|
|Printhead Thermistor<br>Open|1|00000000|X|X|X|X|X|2|X|X|
|Invalid Firmware Config.|1|00000000|X|X|X|X|X|1|X|X|
|Printhead Detection<br>Error|1|00000000|X|X|X|X|X|X|8|X|
|Bad Printhead Element|1|00000000|X|X|X|X|X|X|4|X|
|Motor Over<br>Temperature|1|00000000|X|X|X|X|X|X|2|X|
|Printhead Over<br>Temperature|1|00000000|X|X|X|X|X|X|1|X|
|Cutter Fault|1|00000000|X|X|X|X|X|X|X|8|
|Head Open|1|00000000|X|X|X|X|X|X|X|4|


362


ZPL Commands


**Table 10** Error Flags (~WQES) (Continued)



















|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles**<br>**16-9**|**Nibble8**|** Nibble7**|** Nibble6**|**  Nibble5**|**  Nibble4**|**   Nibble3**|**   Nibble2**|**    Nibble1**|
|Ribbon Out|1|00000000|X|X|X|X|X|X|X|2|
|Media Out|1|00000000|X|X|X|X|X|X|X|1|
|Clear Paper Path Failed1|1|00000000|X|X|X|X|8|X|X|X|
|Paper Feed Error1|1|00000000|X|X|X|X|4|X|X|X|
|Presenter Not Running1|1|00000000|X|X|X|X|2|X|X|X|
|Paper Jam during<br>Retract1|1|00000000|X|X|X|X|1|X|X|X|
|Black Mark not Found1|1|00000000|X|X|X|8|X|X|X|X|
|Black Mark Calabrate<br>Error1|1|00000000|X|X|X|4|X|X|X|X|
|Retract Function timed<br>out1|1|00000000|X|X|X|2|X|X|X|X|
|Paused1|1|00000000|X|X|X|1|X|X|X|X|


1. This error flag is supported only on KR403 printers.


**Table 11** Warning Flags (~WQES)



















|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles**<br>**16-9**|**Nibble8**|** Nibble7**|** Nibble6**|**  Nibble5**|**  Nibble4**|**   Nibble3**|**   Nibble2**|**    Nibble1**|
|No Warning|0|00000000|0|0|0|0|0|0|0|0|
|Warning Present|1|00000000|X|X|X|X|X|X|X|X|
|Paper-near-end Sensor1|1|00000000|X|X|X|X|X|X|X|8|
|Replace Printhead|1|00000000|X|X|X|X|X|X|X|4|
|Clean Printhead|1|00000000|X|X|X|X|X|X|X|2|
|Need to Calibrate Media|1|00000000|X|X|X|X|X|X|X|1|
|Sensor 1 (Paper before<br>head)1|1|00000000|X|X|X|X|X|X|1|X|
|Sensor 2 (Black mark)1|1|00000000|X|X|X|X|X|X|2|X|
|Sensor 3 (Paper after<br>head)1|1|00000000|X|X|X|X|X|X|4|X|
|Sensor 4 (loop ready)1|1|00000000|X|X|X|X|X|X|8|X|
|Sensor 5 (presenter)1|1|00000000|X|X|X|X|X|1|X|X|


363


ZPL Commands


**Table 11** Warning Flags (~WQES) (Continued)







|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles**<br>**16-9**|**Nibble8**|** Nibble7**|** Nibble6**|**  Nibble5**|**  Nibble4**|**   Nibble3**|**   Nibble2**|**    Nibble1**|
|Sensor 6 (retract ready)1|1|00000000|X|X|X|X|X|2|X|X|
|Sensor 7 (in retract)1|1|00000000|X|X|X|X|X|4|X|X|
|Sensor 8 (at bin)1|1|00000000|X|X|X|X|X|8|X|X|


1. This error flag is supported only on KR403 printers.


**~WQ Examples**


This section provides detailed examples of all the available parameters.


**Example:** This example shows how to request the printer’s status.

To request the printer’s status, type `~WQES`

A label similar to this prints out:


In this example, the Printer Status resolves to these conditions:


          - The cover/printhead is open (value = 4).


          - Media is out or not loaded into the printer (value = 1).


          - The printhead needs to be cleaned (value = 2).


          - Error nibble 1 is equal to 5 when the error status values are added together (4+1).


This illustration identifies the printer status definitions:


1 Flag

2 Nibble 16-9

3 Nibble 8-4

4 Nibble 3

5 Nibble 2

6 Nibble 1


364


ZPL Commands


**Example:** This example shows how to request the printer’s status.

To request the printer’s status, type `~WQES`

A label similar to this prints out:


In the example shown above, the Printer Status resolves to the following conditions:

- The cutter has a fault. (value = `8` ).

- Ribbon is out or not loaded into the printer (value = `2` ).

- Media is out or not loaded into the printer (value = `1` ).

- Error byte 1 is equal to `B` when the error status values are added together ( `8 + 2 + 1 =`
`hexadecimal B` ).

**Example:** This is an example of how to print the hardware address of the internal wired print server.

1. To print the hardware address of the internal wired print server, type `~WQHA`

A label similar to this prints out:


**Example:** This is an example of how to print a summary of the printer’s printhead test results.

The `^JT` command is used to initiate printhead testing, set the testing interval, and set the element range
to be tested. For more details see, ^JT.

1. To request a summary of the printer’s printhead test, type `~WQJT`

A label similar to this prints out:


When the printer has printed enough labels to trigger a printhead test, the initial data changes.

1. To request a summary of the printer’s printhead test, type `~WQJT`

A label similar to this prints out:


365


ZPL Commands


This illustration identifies the printhead test field definitions:


1 Element failure

2 Manual (M) or automatic (A) range

3 First test element

4 Last test element

5 Failure count


**Example:** This is an example of how to print the maintenance alert query for the `~WQ` command.

1. To get the current settings, type `~WQMA`

A label similar to this prints out:


**Example:** This is an example of how to use the odometer query for the `~WQ` command. Note that the units
of measure are controlled by the `^MA` command. Also, if the "Early Warning Maintenance State" is turned
"ON" the printer response would also list LAST CLEANED and CURRENT PRINTHEAD LIFE counters.

1. To get the current settings, type `~WQOD`

A label similar to this prints out:


366


ZPL Commands


The units of measure are set to inches.


1. To change the units of measure to centimeters, type:
```
^XA^MA,,,,C
^XZ
```

The units of measure are set to centimeters.

1. To check the settings, type `~WQOD` .

A label similar to this prints out:


**Example:** This is an example of how to print the maintenance information query for the `~WQ` command. Note
that the message is controlled by the `^MI` command.

1. To get the current settings, type `~WQMI`

A label similar to this prints out:


**Example:** This is an example of how to print the printhead life query for the `~WQ` command. Note that the
units of measure are controlled by the `^MA` command.

1. To get the current settings, type `~WQPH`

A label similar to this prints out:


367


ZPL Commands


1 The current life of the print head.

2 Line items 2 through 10 (the example only shows 2 through 3) tracks the measurement for each
time the print head is changed.


**Example:** This is an example of how to print the printer’s Plug and Play string.

1. To print the printer’s Plug and Play string, type `~WQPP`

A label similar to this prints out:


**Example:** This is an example of how to print the printer’s serial number.

1. To get the printer’s serial number, type `~WQSN`

A label similar to this prints out:


**Example:** This is an example of how to print the printer’s USB product ID and BCD release version.

1. To print the printer’s USB product ID and BCD release version, type `~WQUI`

A label similar to this prints out:


368
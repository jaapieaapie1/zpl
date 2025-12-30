# ^HF




ZPL Commands


The `^HF` command sends stored formats to the host.


**Host Format**

**Format:** `^HFd,o,x`







|Parameters|Details|
|---|---|
|`d =` device to recall<br>image|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** `R:`|
|`o =` image name|**Values:** `1` to`8` alphanumeric characters<br>**Default:** if a name is not specified, UNKNOWN is used|
|`x =` extension|**Fixed Value:** `.ZPL`|


**Example:** This example shows the sequence and results.


Using a terminal emulator, you download this code to the printer:

```
^XA
^DFB:FILE1.ZPL
^FO100,100^A0,100
^FDTEST^FS
^XZ

```

Then you send this code to the printer:

```
^XA
^HFB:FILE1.ZPL
^XZ

```

The terminal emulator returns this code:

```
^XA^DFFILE1,
^FO100,100^A0,100^FDTEST^FS
^XZ

```

220




### **^HG**



ZPL Commands


The `^HG` command is used to upload graphics to the host. The graphic image can be stored for future use,
or it can be downloaded to any Zebra printer.


**Host Graphic**


**Format:** ^HGd:o.x


**Parameters**

- d (device location of object)- Values= `R:`, `E:`, `B:`, and `A:;`

`Default=` search priority

- `o (` object name)- Values= `R:`, `E:`, `B:`, and `A` ;

Default =if a name is not specified, UNKNOWN is used

- `x (` extension)- Fixed Value= `.GRF`

**Comments:** For more information on uploading graphics, see .


221


### **^HH**



ZPL Commands


The `^HH` command echoes printer configuration back to the host using a terminal emulator.


**Configuration Label Return**

**Format:** `^HH`

**Example:** This is an example of what is returned to the host when `^XA^HH^XZ` is sent to the printer:


222


ZPL Commands


223


### **~HI**



ZPL Commands


The `~HI` command is designed to be sent from the host to the Zebra printer to retrieve information. Upon
receipt, the printer responds with information on the model, software version, dots-per-millimeter setting,
memory size, and any detected options.


**Host Identification**

**Format:** `~HI`

When the printer receives this command, it returns:

```
XXXXXX,V1.0.0,dpm,000KB,X

```

XXXXXX = model of Zebra printer


V1.0.0 = version of software


dpm = dots/mm


6, 8, 12, or 24 dots/mm printheads


000KB = memory


512KB = 1/2 MB


1024KB = 1 MB


2048KB = 2 MB


4096KB = 4 MB


8192KB = 8 MB


x = recognizable options


only options specific to printer are shown (cutter, options, et cetera.)


224


### **~HM**



ZPL Commands


Sending `~HM` to the printer immediately returns a memory status message to the host. Use this command
whenever you need to know the printer’s RAM status.


**Host RAM Status**

When `~HM` is sent to the Zebra printer, a line of data containing information on the total amount, maximum
amount, and available amount of memory is sent back to the host.

**Format:** `~HM`

**Example:** This example shows when the `~HM` is sent to the printer, a line of data containing three numbers
are sent back to the host. Each set of numbers is identified and explained in the table that follows:

## 1 2 3


1 The total amount of RAM (in kilobytes) installed in the printer. In this example, the printer has
1024K RAM installed.

2 The maximum amount of RAM (in kilobytes) available to the user. In this example, the printer has
a maximum of 780K RAM available.

3 The amount of RAM (in kilobytes) currently available to the user. In this example, there is 780K
of RAM in the printer currently available to the user.


**Comments:** Memory taken up by bitmaps is included in the currently available memory value (due to
`^MCN` ).

Downloading a graphic image, fonts, or saving a bitmap affects only the amount of RAM. The total amount
of RAM and maximum amount of RAM does not change after the printer is turned on.


225


### **~HQ**



ZPL Commands


The `~HQ` command group causes the printer to send information back to the host.


**Host Query**

**Format:** `~HQquery-type`

|Parameter|Details|
|---|---|
|`query-type`|For detailed examples of these parameters, see~HQ Examples on page<br>228.<br>**Values:**<br>•<br>`ES =` requests the printer’s status - seeTable 8    Error Flags (~HQES)<br>on page 226 andTable 9    Warning Flags (~HQES) on page 227<br>•<br>`HA =` hardware address of the internal wired print server<br>•<br>`JT =` requests a summary of the printer’s printhead test results<br>•<br>`MA =` maintenance alert settings<br>•<br>`MI =` maintenance information<br>•<br>`OD =` odometer<br>•<br>`PH =` printhead life history<br>•<br>`PP =` printer’s Plug and Play string<br>•<br>`SN =` printer’s serial number<br>•<br>`UI =` USB product ID and BDC release version<br>•<br>**Default:** must be an accepted value or the command is ignored|



**Comments:** The response to the `~HQ` command starts with STX, a CR LF is inserted between each line, and
the response ends with ETX.



**Table 8** Error Flags (~HQES)





































|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles**<br>**16-9**|**Nibble**<br>**8**|**Nibble**<br>**7**|**Nibble**<br>**6**|**Nibble**<br>**5**|**Nibble**<br>**4**|**Nibble**<br>**3**|**Nibble**<br>**2**|**Nibble**<br>**1**|
|No Error|0|00000000|0|0|0|0|0|0|0|0|
|Error Present|1|00000000|X|X|X|X|X|X|X|X|
|Printhead Thermistor<br>Open|1|00000000|X|X|X|X|X|2|X|X|
|Invalid Firmware Config.|1|00000000|X|X|X|X|X|1|X|X|
|Printhead Detection<br>Error|1|00000000|X|X|X|X|X|X|8|X|
|Bad Printhead Element|1|00000000|X|X|X|X|X|X|4|X|
|Motor Over<br>Temperature|1|00000000|X|X|X|X|X|X|2|X|


226


ZPL Commands


**Table 8** Error Flags (~HQES) (Continued)





































|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles**<br>**16-9**|**Nibble**<br>**8**|**Nibble**<br>**7**|**Nibble**<br>**6**|**Nibble**<br>**5**|**Nibble**<br>**4**|**Nibble**<br>**3**|**Nibble**<br>**2**|**Nibble**<br>**1**|
|Printhead Over<br>Temperature|1|00000000|X|X|X|X|X|X|1|X|
|Cutter Fault|1|00000000|X|X|X|X|X|X|X|8|
|Head Open|1|00000000|X|X|X|X|X|X|X|4|
|Ribbon Out|1|00000000|X|X|X|X|X|X|X|2|
|Media Out|1|00000000|X|X|X|X|X|X|X|1|
|Clear Paper Path Failed1|1|00000000|X|X|X|X|8|X|X|X|
|Paper Feed Error1|1|00000000|X|X|X|X|4|X|X|X|
|Presenter Not Running1|1|00000000|X|X|X|X|2|X|X|X|
|Paper Jam during<br>Retract1|1|00000000|X|X|X|X|1|X|X|X|
|Black Mark not Found1|1|00000000|X|X|X|8|X|X|X|X|
|Black Mark Calibrate<br>Error1|1|00000000|X|X|X|4|X|X|X|X|
|Retract Function timed<br>out1|1|00000000|X|X|X|2|X|X|X|X|
|Paused|1|00000000|X|X|X|1|X|X|X|X|


1. This error flag is supported only on KR403 printers.


**Table 9** Warning Flags (~HQES)



























|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles**<br>**16-9**|**Nibble**<br>**8**|**Nibble**<br>**7**|**Nibble**<br>**6**|**Nibble**<br>**5**|**Nibble**<br>**4**|**Nibble**<br>**3**|**Nibble**<br>**2**|**Nibble**<br>**  1**|
|No Warning|0|00000000|0|0|0|0|0|0|0|0|
|Warning Present|1|00000000|X|X|X|X|X|X|X|X|
|Paper-near-end Sensor1|1|00000000|X|X|X|X|X|X|X|8|
|Replace Printhead|1|00000000|X|X|X|X|X|X|X|4|
|Clean Printhead|1|00000000|X|X|X|X|X|X|X|2|
|Need to Calibrate Media|1|00000000|X|X|X|X|X|X|X|1|
|Sensor 1 (Paper before<br>head)1|1|00000000|X|X|X|X|X|X|1|X|
|Sensor 2 (Black mark)1|1|00000000|X|X|X|X|X|X|2|X|


227


ZPL Commands


**Table 9** Warning Flags (~HQES) (Continued)

























|Error Flags|Flag|Group 2|Group 1 (X = Value can be any hexadecimal number [0-9, A-F])|Col5|Col6|Col7|Col8|Col9|Col10|Col11|
|---|---|---|---|---|---|---|---|---|---|---|
|**Error Flags**|**Flag**|**Nibbles**<br>**16-9**|**Nibble**<br>**8**|**Nibble**<br>**7**|**Nibble**<br>**6**|**Nibble**<br>**5**|**Nibble**<br>**4**|**Nibble**<br>**3**|**Nibble**<br>**2**|**Nibble**<br>**  1**|
|Sensor 3 (Paper after<br>head)1|1|00000000|X|X|X|X|X|X|4|X|
|Sensor 4 (loop ready)1|1|00000000|X|X|X|X|X|X|8|X|
|Sensor 5 (presenter)1|1|00000000|X|X|X|X|X|1|X|X|
|Sensor 6 (retract ready)1|1|00000000|X|X|X|X|X|2|X|X|
|Sensor 7 (in retract)1|1|00000000|X|X|X|X|X|4|X|X|
|Sensor 8 (at bin)1|1|00000000|X|X|X|X|X|8|X|X|


1. This error flag is supported only on KR403 printers.


**~HQ Examples**


This section provides detail examples of all the available parameters.


**Example:** This example shows how to request the printer’s status.

To request the printer’s status, type `~HQES`

The printer responds with data similar to this:

```
       PRINTER STATUS
       ERRORS:     1 00000000 00000005
       WARNINGS:    1 00000000 00000002

```

In this example, the Printer Status resolves to these conditions:


          - The cover/printhead is open (value = 4).


          - Media is out or not loaded into the printer (value = 1).


          - The printhead needs to be cleaned (value = 2).


          - Error nibble 1 is equal to 5 when the error status values are added together (4 + 1).


This illustration identifies the printer status definitions:


1 3 5


PRINTER STATUS



ERRORS:
WARNINGS:


1 Flag



1 00000000 00000005

1 00000000 00000002


2 4 6


228


ZPL Commands


2 Nibble 16-9

3 Nibble 8-4

4 Nibble 3

5 Nibble 2

6 Nibble 1


**Example:** This example shows how the printer responds when the printer receives the `~HQES` command:

To see how the printer responds, type `~HQES`

The printer responds with data similar to this:

```
PRINTER STATUS
ERRORS:  1 00000000 0000000B
WARNINGS: 0 00000000 00000000

```

In this example, the printer status resolves to the following conditions:

- The cutter has a fault (value = `8` ).

- Ribbon is out or not loaded into the printer (value = `2` ).

- Media is out or not loaded into the printer (value = `1` ).

- Error byte 1 is equal to `B` when the error status values are added together ( `8 + 2 + 1 =`
`hexadecimal B` ).

**Example:** This is an example of how to retrieve the hardware address of the internal wired print server.

To get the hardware address of the internal wired print server, type `~HQHA`

The printer responds with data similar to this:

```
MAC ADDRESS
00:07:4d:2c:e0:7a

```

**Example:** This is an example of how to request a summary of the printer’s printhead test results.

The `^JT` command is used to initiate printhead testing, set the testing interval, and set the element range
to be tested. For more details see,^JT.

To request a summary of the printer’s printhead test, type `~HQJT`

The printer responds with data similar to this:

```
PRINT HEAD TEST RESULTS
0,A,0000,0000,0000

```

When the printer has printed enough labels to trigger a printhead test, the initial data changes.

To request a summary of the printer’s printhead test, type `~HQJT` The printer responds with data similar to
this:

```
PRINT HEAD TEST RESULTS:
0,A,0015,0367,0000

```

229


ZPL Commands


This illustration identifies the printhead test field definitions:


0, A,0000,0000,0000


1 2 3 4 5


1 Element failure

2 Manual (M) or automatic (A) range

3 First test element

4 Last test element

5 Failure count


**Example:** This is an example of how to use the maintenance alert query for the `~HQ` command.

To get the current settings, type `~HQMA`

The printer responds with data similar to this:

```
~HQMA
MAINTENANCE ALERT SETTINGS
HEAD REPLACEMENT INTERVAL:   1 km
HEAD REPLACEMENT FREQUENCY:   0 M
HEAD CLEANING INTERVAL:     0 M
HEAD CLEANING FREQUENCY:    0 M
PRINT REPLACEMENT ALERT:     NO
PRINT CLEANING ALERT:      NO
UNITS:              C

```

**Example:** This is an example of how to use the maintenance information query for the `~HQ` command.

Note that the message is controlled by the `^MI` command.

To get the current settings, type `~HQMI`

The printer responds with data similar to this:

```
MAINTENANCE ALERT MESSAGES
CLEAN: PLEASE CLEAN PRINT HEAD
REPLACE: PLEASE REPLACE PRINT HEAD

```

**Example:** This is an example of how to use the odometer query for the `~HQ` command.

Note that the units of measure are controlled by the `^MA` command. Also, if the "Early Warning
Maintenance State" is turned "ON" the printer response would also list LAST CLEANED and CURRENT
PRINTHEAD LIFE counters.

To get the current settings, type `~HQOD`

The printer responds with data similar to this:

```
PRINT METERS
TOTAL NONRESETTABLE:     8560 "
USER RESETTABLE CNTR1:     9 "

```

230




ZPL Commands

```
USER RESETTABLE CNTR2:    8560 "

```

The units of measure are set to inches.


To change the units of measure to centimeters, type:

```
^XA^MA,,,,C
^XZ

```

The units of measure are set to centimeters.

a. To check the settings, type `~HQOD`

The printer responds with data similar to this:

```
PRINT METERS
TOTAL NONRESETTABLE:    21744 cm
USER RESETTABLE CNTR1:     24 cm
USER RESETTABLE CNTR2:   21744 cm

```

**Example:** This is an example of how to use the printhead life query for the `~HQ` command.

Note that the units of measure are controlled by the `^MA` command.

To get the current settings, type `~HQPH`

The printer responds with data similar to this:

```
LAST CLEANED: 257 "
HEAD LIFE HISTORY
#  DISTANCE
1:    257 "
2:    1489 "
3:    7070 "

```

line 1 The current life of the print head.

lines 2–10 Tracks the measurement for each time the print head is changed. (The example only
shows lines 2 and 3.)


**Example:** This is an example of how to request the printer’s Plug and Play string.

To request the printer’s Plug and Play string, type `~HQPP`

The printer responds with data similar to this:

```
PLUG AND PLAY MESSAGES
MFG: Zebra Technologies
CMD: ZPL
MDL: GX420t

```

**Example:** This is an example of how to retrieve the printer’s serial number.

To get the printer’s serial number, type `~HQSN`

The printer responds with data similar to this:


231


ZPL Commands

```
SERIAL NUMBER
41A06440023

```

**Example:** This is an example of how to retrieve the printer’s USB product ID and BCD release version.

To get the printer’s USB product ID and BCD release version, type `~HQUI`

The printer responds with data similar to this:

```
USB INFORMATION
PID:             0085
RELEASE VERSION:       15.01

```

232
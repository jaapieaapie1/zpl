# Real Time Clock


**Real Time Clock**


This appendix contains the information needed to install, program, and operate the Real Time Clock (RTC)
option. This hardware option is available as either a factory-installed or option in specific
field-installable
printer products manufactured and sold by Zebra Technologies Corporation.


The Real Time Clock option is currently available for following printers, and requires that the firmware
version shown is installed on that printer.























|Supported Zebra Printer and Print Engine Models|Col2|Requires this Firmware<br>Version or Later|
|---|---|---|
|105SL printers that meet any one of the following criteria:<br>•<br>if the printer was manufactured after April 2006<br>•<br>if the RTC Date and the RTC Time fields are printed on the configuration<br>label<br>•<br>if the RTC Date and the RTC Time appear on the LCD display|105SL printers that meet any one of the following criteria:<br>•<br>if the printer was manufactured after April 2006<br>•<br>if the RTC Date and the RTC Time fields are printed on the configuration<br>label<br>•<br>if the RTC Date and the RTC Time appear on the LCD display|60.13.0.13Z|
|DA402 & T402 printers|DA402 & T402 printers|32.8.4|
|LP2844-Z, TPL2844-Z, and TPL3844-Z printers|LP2844-Z, TPL2844-Z, and TPL3844-Z printers|45.10.x|
|PAX Series print<br>engines|170PAX2 print engines|29.9.x or 31.9.x|
|PAX Series print<br>engines|110PAX3 print engines using Standard Font|34.10.x|
|PAX Series print<br>engines|110PAX3 print engines using TT Font|49.10.x|
|PAX Series print<br>engines|116PAX3 print engines using TT Font|35.10.x|
|PAX Series print<br>engines|170PAX3 print engines using Standard Font|37.10.x|
|PAX Series print<br>engines|170PAX3 print engines using TT Font|38.10.x|
|PAX Series print<br>engines|110PAX4 and 170PAX4 print engines|60.13.0.12|
|S4M printers (field-installable kit)|S4M printers (field-installable kit)|50.13.x|
|S600 printers|S600 printers|27.10.3|
|Xi Series<br>printers|90XiII, 140XiII, 170XiII & 220XiII printers|18.9.x|
|Xi Series<br>printers|90XiIII, 96XiIII, 140XiIII, 170XiIII, and 220XiIII printers|33.10.0|
|Xi Series<br>printers|90XiIIIPlus, 96XiIIIPlus, 140XiIIIPlus, 170XiIIIPlus, and<br>220XiIIIPlus printers|60.13.0.12|
|Z4Mplus and Z6Mplus printers|Z4Mplus and Z6Mplus printers|60.13.0.12|
|ZM400/ZM600/RZ400/RZ600 printers|ZM400/ZM600/RZ400/RZ600 printers|53.15.xZ|


1595


Real Time Clock

## **Control Panel Programming**


New parameters for the Real Time Clock have been added to the Control Panel Configuration. These
parameters are located immediately following the **FORMAT CONVERT** prompt. Refer to the printer/print
engine User Guide for complete configuration information.


          - X.9.x firmware added the parameters to XiII printers and 170PAX/170PAX2 print engines.


          - X.10.x firmware added the parameters to XiIII printers and PAX3 print engines.


          - X.13.x firmware added the parameters to XiIIIPlus printers, PAX4 print engines.


          - X.13.0.13Z firmware added the parameters to 105SL printers.

## **Real Time Clock Parameters**


The parameters listed on the following pages are added to the Control Panel Configuration prompts only
when both the Real Time Clock hardware option and the appropriate version of firmware are installed.


          - X.9.x or later firmware installed in the XiII series printers or the 170PAX/170PAX2 series print engines.


          - X.10.x or later firmware installed in the XiIII series printers or the PAX3 series print engines.


          - X.13.0.13Z or later firmware installed in the 105SL printers


          - X.13.x or later firmware installed in the XiIIIPlus series printers, the PAX4 series print engines, or the S4M
printers.


The RTC ZPL II commands apply to all printers/print engines with the Real Time Clock hardware option and
proper firmware.

### **Idle Display**


Selects the printer/print engine Idle Display format and the method of displaying the time/date information.


This parameter also affects the Configuration Label printout and the RTC DATE and RTC TIME formats.


Selections


          - FW VERSION


          - MM/DD/YY 24HR


          - MM/DD/YY 12HR


          - DD/MM/YY 24HR


          - DD/MM/YY 12HR


If FW VERSION is selected, the format on the Configuration Label and on the RTC DATE and RTC TIME
parameters is MM/DD/YY 24HR.

### **RTC Date**


Allows entry of the RTC date in the format selected by the **IDLE DISPLAY** parameter.


**NOTE:** The RTC parameters are password-protected. Refer to your printer’s user guide for
specific instructions on accessing and modifying printer parameters.


1596


Real Time Clock







|Printer Model|Action|
|---|---|
|PAX Series print<br>engines, Xi Series<br>printers, and<br>105SL printers|Use the LEFT oval key to select the position to be adjusted<br>Then, use the RIGHT oval key to select the correct value for that position.|
|Z4Mplus and<br>Z6Mplus printers|Press**SELECT** to select the parameter.<br>Use the**MINUS** (-) key to select the position to be adjusted<br>Then, use the**PLUS** (+) key to select the correct value for that position.<br>Press**SELECT** to accept any changes and deselect the parameter.|
|S4M printer|Press ENTER. The printer displays the current RTC date.<br>Modify the values as follows:<br>•<br>Press the right arrow to move to the next digit position.<br>•<br>To increase the value, press the up arrow.<br>•<br>To decrease the value, press the down arrow.<br>Press**ENTER** to accept the value shown.|


**NOTE:** Invalid dates, such as 2/30/1999, may be entered, but they will not be saved.

### **RTC Time**


Allows entry of the RTC time in the format selected by the **IDLE DISPLAY** parameter.


**NOTE:** The RTC parameters are password-protected. Refer to your printer’s user guide for
specific instructions on accessing and modifying printer parameters.







|Printer Model|Action|
|---|---|
|PAX Series print<br>engines, Xi Series<br>printers, and<br>105SL printers|Use the LEFT oval key to select the position to be adjusted<br>Use the RIGHT oval key to select the correct value for that position.|
|Z4Mplus and<br>Z6Mplus printers|Press**SELECT** to select the parameter.<br>Use the**MINUS** (-) key to select the position to be adjusted.<br>Use the**PLUS** (+) key to select the correct value for that position.<br>Press**SELECT** to accept any changes and deselect the parameter.|
|S4M printer|Press ENTER. The printer displays the current RTC date.<br>Modify the values as follows:<br>•<br>Press the right arrow to move to the next digit position.<br>•<br>To increase the value, press the up arrow.<br>•<br>To decrease the value, press the down arrow.<br>Press**ENTER** to accept the value shown.|


1597


Real Time Clock

### **RTC General Information**


The Real Time Clock commands are only applicable if the Real Time Clock option is installed in the printer.
For those printers with an LCD control panel display, additional control panel configuration parameters are
also included.

The ZPL II Field Clock `^FC` command is used to specify the character for the primary,
clock-indicator
secondary, and third clocks. This command must be included within each label field command string
whenever the date or time clock values are required within the field. No date or time clock information can
be printed in a label field unless this command is included. The `^FC` command can now be combined with
the `^SN` command in V60.13.0.10 and later.

A clock-indicator can be any printable character except the ZPL II Format Prefix, Control Prefix, or Delimiter
characters. The default value for the primary clock-indicator is the percent sign `%` . The secondary and third
clock-indicators have no defaults and must be specified in order for that clock to be used.

The Field Data `^FD` command has been expanded to recognize the clock-indicators and associated
command characters, and to replace them during the printing process with the corresponding time or
date parameter. For example, if the primary clock-indicator is the percent sign `%`, then during printing, the
character sequence `%H` in the `^FD` statement would be replaced by the 2-digit current hour.


**NOTE:** If the Real Time Clock is not installed, or the `^FC` command has not preceded the `^FD`
statement, no replacement would occur. In this case, the characters `%H` would print as text on the
label.


The name of the day of the week, the name of the month, and the AM or PM designation can also be
inserted in place of a specific clock-indicator/command character sequence. This table lists command
characters and their functions.


**Table 39** Command Characters







|Command<br>Character|Function|
|---|---|
|`%a`|is replaced by the abbreviated weekday name|
|`%A`|is replaced by the weekday name|
|`%b`|is replaced by the abbreviated month name|
|`%B`|is replaced by the month name|
|`%d`|is replaced by the day of the month number, 01 to 31|
|`%H`|is replaced by the hour of the day (military), 00 to 23|
|`%I`|is replaced by the hour of the day (civilian), 01 to 12|
|`%j`|is replaced by the day of the year, 001 to 366|
|`%m`|is replaced by the month number, 01 to 12|
|`%M`|is replaced by the minute, 00 to 59|
|`%p`|is replaced by the AM or PM designation|
|`%S`|is replaced by the seconds, 00 to 59|
|`%U`|is replaced by the week# of the year, 00 to 53, Sunday is 1st day1|
|`%W`|is replaced by the week# of the year, 00 to 53, Monday is 1st day2|
|`%w`|is replaced by the day# of the week, 00 (Sunday) to 06 (Saturday)|


1598


Real Time Clock


**Table 39** Command Characters (Continued)





|Command<br>Character|Function|
|---|---|
|`%y`|is replaced by the 2 digits of the year, 00 to 99|
|`%Y`|is replaced by the full 4 digit year number—where% is the specified clock-indicator<br>character|


1. `%U` establishes Sunday as the first day of the year.

2. `%W` establishes Monday as the first day of the year.

The Set Offset `^SO` command permits the printing of specific times and dates relative to the primary
clock. The secondary (or third) clock is enabled when secondary (or third) offsets are entered using this
command. The secondary (or third) clock time and date are determined by adding the offsets to the current
clock reading.

One `^SO` command is required to set the secondary offset; an additional `^SO` command is required for a
third offset. The offsets remain until changed or until the printer is either powered down or reset.


**NOTE:** Only dates from January 1, 1998 to December 31, 2097 are supported. Setting the offsets
to values that result in dates outside this range is not recommended and may have unexpected
results.


The Set Mode/Language (see ^SL) command is used to select the language the days of the week and the
months are printed in. This command also sets the printing mode, which can be `S` for START TIME, `T` for
TIME NOW, or a Numeric Value for the time accuracy. In START TIME mode, the time printed on the label is
the time that is read from the Real Time Clock when the label formatting begins (when the `^XA` command
is received by the printer). In TIME NOW mode, the time printed on the label is the time that is read from
the Real Time Clock when the label is placed in the queue to be printed. In Numeric Value mode, a time
accuracy tolerance can be specified.

### **First Day of the Week Affects Calendar Week**


The `%U` and `%W` commands set the first day of the week. The week numbering starts at the beginning of the
year with Week 01 representing the first full week of the year. Any day(s) before that established first day of
the week are part of the Week 00. The following examples show how setting different days as the first day
of the week affect the calendar week.


**IMPORTANT:** The `%U` and `%W` commands determine the numbering for all weeks in the year.


January, 2005 with Week 00

Set Sunday as the first day of the week using the `%U` command. In this example, notice that Saturday,
January 1st is Week 00 and Sunday, January 2nd begins Week 01.


1599


Real Time Clock


January, 2005 with Week 00

Set Monday as the first day of the week using the `%W` command. In this example, notice that Saturday,
January 1st and Sunday, January 2nd are Week 00 and Monday, January 3rd begins Week 01.


January, 2006 without Week 00

Set Sunday as the first day of the week using the `%U` command. Since 2006 begins on a Sunday, there is
no Week 00 in this example.

### **Time and Date Precision**


As of V60.13.0.1 firmware, the `^CO` command is now ignored. While the S4M printer has a lower firmware
version number (V50.x), its firmware was recently released and follows the rule to ignore the `^CO`
command.


The time and date placed in a label field is determined at the time the label bitmap is created by the printer
(start time mode). If a batch of labels is formatted, the date and time will be the same for all labels in the
batch. If the printer is paused during the printing process and remains in that state for a period of time,
when printing resumes, the time and date will still be the same as when the batch was first started.


If more precise time and date stamps are required on versions prior to V60, follow the process below. For
versions after V60, use the Numeric Value mode as shown in ^SL.


Cycle the printer/print engine power Off (O) and On (l) to clear the memory before performing the steps
below.



1. Print a Memory Usage Label (^XA^WD*:*.*^XZ) and note the following value:
Available RAM (in BYTES)


2. Print a Configuration Label and note these values: Printer “Print Width” (in DOTS)
(NOT the Label Width)



(A) __________


(B) __________



Label Length (in DOTS) (C) __________


1600




Real Time Clock


3. Determine the desired maximum number of queued labels with the same Time
and Date value.



(D) __________



**NOTE:** Increasing the number of queued labels will improve throughput performance, but Real
Time Clock values will be less accurate. Two is usually a good compromise.



4. Substitute the values for B through D from the previous page into the following
formula:

The “label queue” memory required (in BYTES)(B x C x D)/8 =


5. Substitute the values for A and E into the following formula: The ^CO command
memory required (in KBYTES) (A-E)/1024)-5=



(E) __________


(F) __________



**NOTE:** If the value of (F) is less than zero, then no ^CO command is needed. If the value of (F) is
greater than zero, use the integer portion in the ^CO command.


Available RAM (A) = 71478 BYTES

Print Width (B) = 832 DOTS

Label Length (C) = 1000 DOTS

Max Labels Queued (D) = 2

Then —

The label queue memory required (E) =(B x C x D)/8 = 208000 BYTES

And —

The ^CO command memory required (F) = (71478-208000)/1024)-5=489.87 KBYTES

Therefore, the correct `^CO` command string to add to the label format would be:

^XA^COY,489^XZ


This command string will cause 489 KBYTES to be set aside as Font Memory and make it unavailable as
label format memory. The memory remaining will only allow two labels to be formatted at one time, and the
time and date will be more precise for those two labels.

### **ZPL II Samples**


The ZPL II scripts shown on this page establish the initial settings for the date and time clock. The script
below then references these settings to provide the output shown in Figure 26  Printed Result of the
Above ZPL II Script on page 1602.


Setting the date and time for the Real Time Clock only needs to be done once. The date and time are
maintained by an on-board battery when the printer is reset or the printer is turned Off (O).


To set the date and time to April 23, 2005 at 2:30pm, the following command string should be sent to the
printer:

```
     ^XA
     ^ST04,23,2005,02,30,0,P^FS
     ^XZ

```

To initialize the Real Time Clock and set up two offset values (offset #2 set to 3 months and 1 hour in the
future, offset #3 set to 1 year in the past), the following command sequence should be sent to the printer:


1601


Real Time Clock

```
^XA
^SL
^SO2,3,0,0,1,0,0^FS
^SO3,0,0,-1,0,0,0^FS
^XZ

```

The above ZPL II scripts initialize the RTC date and time and must be sent to a printer to provide proper
date and time parameters for the ZPL II script below.


The following ZPL II script illustrates the various methods of printing the date and time initialized in the
script above within separate fields on continuous media. Figure 26  Printed Result of the Above ZPL II
Script on page 1602illustrates the printout of this script on a label.

For the below example, the `^FC` command delimiters are:

`%` Primary clock indicator `{` Secondary clock indicator `#` Third clock indicator

```
^XA
^LL175
^FO10,025^AD^FC%,{,#^FD1: Mil: %H:%M:%S Civ: %I:%M:%S %p^FS
^FO10,050^AD^FC%,{,#^FD2: Mil: {H:{M:{S Civ: {I:{M:{S {p^FS
^FO10,075^AD^FC%,{,#^FD3: Mil: #H:#M:#S Civ: #I:#M:#S #p^FS
^FO10,100^AD^FC%,{,#^FD1: On %A, %B %d
, %Y (%a, %m/%d/%y, %d %b %Y).^FS
^FO10,125^AD^FC%,{,#^FD2: On {A, {B {d, {Y (
{a, {m/{d/{y, {d {b {Y).^FS
^FO10,150^AD^FC%,{,#^FD3: On #A, #B #d, #Y (
#a, #m/#d/#y, #d #b #Y).^FS
^XZ

```

**Figure 26** Printed Result of the Above ZPL II Script


The following are examples of the time stamp using the `^SL1` and `^SL5` at 2 ips and 10 ips for the
Enhanced Real Time Clock (V60.13.0.10 and later).


**NOTE:** They show the variation of time due to print speed and label complexity.

```
^XA
^SL1^FS

```

1602


Real Time Clock

```
^FO187,184^A0N,101,121^FC%^FD%H:%M:%S^FS
^PQ10
^XZ

```

**Figure 27** Example of ^SL1, 2 ips and 10 ips


1 Label 1 6 Label 6

2 Label 2 7 Label 7

3 Label 3 8 Label 8

4 Label 4 9 Label 9

5 Label 5 10 Label 10

```
^XA

```

1603


Real Time Clock

```
^SL5^FS
^FO187,184^A0N,101,121^FC%^FD%H:%M:%S^FS
^PQ10
^XZ

```

**Figure 28** Example of ^SL5, 2 ips and 10 ips


1 Label 1 6 Label 6

2 Label 2 7 Label 7

3 Label 3 8 Label 8

4 Label 4 9 Label 9

5 Label 5 10 Label 10


1604
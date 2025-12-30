# ^RS




ZPL RFID Commands


Use this command to set up RFID parameters including tag type; programming position; and error handling,
such as setting the number of labels that will be attempted if an error occurs.


**Set Up RFID Parameters**


For example, if an RFID label fails to program correctly or if the transponder cannot be detected, the printer
ejects the label and prints `VOID` across it. The printer will try to print another label with the same data and
format for the number of labels specified (parameter `n` ). If the problem persists, the printer follows the error
handling instructions specified by the error handling parameter (parameter `e` ): the printer may remove the
problematic format from the print queue and proceed with the next format (if one exists in the buffer), or it
may place the printer in Pause or Error mode.


**IMPORTANT:** Use care when using this command in combination with `^RF` for reading tag
data. Use care when using this command in combination with `^RT` or `^RF` for reading tag data.
Problems can occur if the data read from the tag is going to be printed on the label. Any data
read from the tag must be positioned to be printed above the read/write position. Failure to do
this will prevent read data from being printed on the label.

**Format:** `^RSt,p,v,n,e,a,c,s`





|Parameters|Details|
|---|---|
|`t` = tag<br>type|**Values:**<br>`8` = EPC Class 1, Generation 2 (Gen 2)<br>**Default:**`8`—Gen 2 is the only tag type supported by current RFID printers. For tag types<br>supported by older printers, refer to the original RFID Programming Guide.|


432


ZPL RFID Commands













|Parameters|Details|
|---|---|
|`p` = read/<br>write<br>position<br>of the<br>tag<br>(programm<br>position)|ing<br>This parameter sets the read/write position of the tag.<br>**IMPORTANT:** If a label format specifies a value for the programming position, this<br>value will be used for the programming position for all labels until a new position is<br>specified or until the tag calibration procedure is run.<br>**For Link-OS printers:**<br>**Values:**<br>`F0` to`Fxxx`<br>(where`xxx` is the label length in millimeters or`999`, whichever is less) The printer prints the<br>first part of a label until it reaches the specified distance and then begins programming. After<br>programming, the printer prints the remainder of the label.<br>`B0` to`B30`<br>The printer backfeeds the label for the specified distance and then begins programming. To<br>account for the backfeed, allow empty media liner to extend out of the front of the printer<br>when using a backward programming position.<br>`up` = move to the next value<br>`down` = move to the previous value<br>**Default:**`F0` (which moves the leading edge of the label to the print line)<br>**For older RFID printers:**<br>**Values:**<br>**Absolute Mode** (all firmware versions):<br>`xxxx` =`0` to label length (in dot rows). Move the media to the specified position`xxxx` on the<br>label, measured in dot rows from the label top, before encoding. Set to`0` (no movement) if<br>the tag is already in the effective area without moving the media.<br>**Relative Mode** (firmware versions V53.17.6 and later):<br>`F0` to`Fxxx`<br>(where`xxx` is the label length in millimeters or`999`, whichever is less). The printer prints the<br>first part of a label until it reaches the specified distance and then begins programming. After<br>programming, the printer prints the remainder of the label.<br>`B0` to`B30`<br>(Does not apply to the RP4T printer.)<br>The printer backfeeds the label for the specified distance and then begins programming. To<br>account for the backfeed, allow empty media liner to extend out of the front of the printer<br>when using a backward programming position.<br>Default:<br>For the R2844-Z and RPAX:`0` (no movement)<br>For printers using V53.17.6, V74.19.6Z, and later:`F0`<br>(which moves the leading edge of the label to the print line)<br>All others: label length minus 1 mm (1/16 in.)|
|`v` =<br>length<br>of void<br>printout|Sets the length of the void printout in vertical (Y axis) dot rows.<br>**Values:**`0` to label length<br>**Default:**label length|


433


ZPL RFID Commands





|Parameters|Details|
|---|---|
|`n` =<br>number<br>of labels<br>to try<br>encoding|The number of labels that will be attempted in case of read/encode failure.<br>**Values:**`1` to`10`<br>**Default:**`3`|
|`e` = error<br>handling|If an error persists after the specified number of labels are tried, perform this error handling<br>action.<br>**Values:**<br>`N =`No action (printer drops the label format causing the error and moves to the next<br>queued label)<br>`P =`Place printer in Pause mode (label format stays in the queue until the user cancels)<br>`E =`Place printer in Error mode (label format stays in the queue until the user cancels)<br>**Defaults:**`N`<br>**NOTE:** You can set the printer to send an error message to the host for each<br>failure. To enable or disable this unsolicited error message, refer to the`^SX` and<br>`^SQ` ZPL commands. Use`V` for the condition type for an RFID error.|
|`a` =<br>signals<br>on<br>applicator|**NOTE:** This parameter applies only to older RFID printers that have an applicator<br>board. This parameter does not apply to the R2844-Z or to Link-OS printers. For<br>the R4Mplus, this parameter applies only to printers with firmware version SP994X<br>(R4Mplus European version).<br>**Single Signal Mode**<br>In this mode, one start print signal starts printing. Then, at the program position (parameter<br>p), the printer automatically stops and encodes the tag. Printing continues, and a single end<br>print signal signifies the completion of the label.<br>**Double Signal Mode**<br>With RFID, when there is a non-zero program position, the label is logically split into two<br>parts. The first part is printed, the tag encodes, and then the second part prints. If this<br>parameter is set to “D,” then the label is split into two and requires both portions of the label<br>to be controlled by the applicator. This means that a start print signal triggers the first portion<br>of the label, and then when the printer reaches the RFID program position (and the motor<br>stops), an end print signal is provided. In this mode, a second start print signal is required to<br>print the rest of the label. When the label is complete, a final end print signal is provided.<br>**NOTE:** If parameter p is zero, then single signal mode is used (parameter ignored).<br>If p is F0 (or B0) with backfeed-after, then single signal mode is used (parameter<br>ignored).<br>**Values:**<br>`S =`single signal<br>`D =`double signal (For the R110PAX4, Double mode will work only if the read/write position<br>is changed from the default of zero.)<br>**Default:**`S`|
|`c` =<br>reserved|Not applicable.|


434


ZPL RFID Commands






|Parameters|Details|
|---|---|
|`s` = void<br>print<br>speed|**NOTE:** This parameter is not supported on all printer models.<br>If a label is voided, the speed at which “VOID” will be printed across the label.<br>**Values:**any valid print speed<br>**Default:**the printer’s maximum print speed|



**Example:** The following are examples of Absolute Mode and Relative Mode for the tag position parameter
(parameter `p` ).

**Absolute Mode**

`^RS,520` sets the encode position at 520 dots from the top edge of the label.

`^RS,0` programs the tag without moving the media.

**Relative Mode**

`^RS,F1` sets the encode position 1 mm forward from the leading edge of the label.

`^RS,B10` sets the encode position 10 mm backwards from the leading edge of the label.

`^RS,F0` sets the encode position at the leading edge of the label.

`^RS,B0` sets the encode position at the leading edge of the label.

**Example:** The following shows the difference between absolute and relative programming positions for the
tag position parameter (parameter `p` ) with a 6-inch (152-mm, 1216-dot) label length. The end results are that
the tag is programmed with the label in the same position.


435


ZPL RFID Commands


1 `^RS,496`, Absolute Mode, 496 dots from the top of the label

2 `^RS,F90`, Relative Mode, 90 mm from the leading edge of the label


436
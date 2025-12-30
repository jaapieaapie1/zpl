# Advanced Techniques


**Advanced Techniques**


This section presents information and commands for using advanced techniques, such as special effects,
serialized data fields, control commands, program delimiters, communications, and memory cards.

## **Special Effects for Print Fields**


This section lists some special effect for print fields.

          - Reverse Printing a Field - The `^FR` (Field Reverse Print) command allows a field to appear as white over
black or black over white. When printing a field, the `^FR` command indicates that it will print the field the
opposite of its background color.

          - Reverse Printing a Label - The `^LR` (Label Reverse Print) command reverses the printing of all fields in
the label format. It allows a field to appear as white over black or black over white. `^LR` functions like
`^FR`, but it applies to all fields in a label. The `^LR` command remains active until turned off.

          - Printing a Mirror Image - The `^PM` (Print Mirror Image of Label) command prints the entire printable area
of the label as a mirror image. This command flips the image from left to right.

          - Printing a Label Inverted 180 Degrees - The `^PO` (Print Orientation) command inverts the label format
180 degrees. In essence, the label is printed upside down.

## **Serialized Data**


The `^SN` (Serialization Data) command allows the printer to index data fields by a selected increment or
decrement value (that is, make the data fields increase or decrease by a specified value) each time a label
is printed.


This can be performed on up to 100 to 150 fields in a given format and can be performed on both
alphanumeric and bar code fields. A maximum of 12 of the right-most integers are subject to indexing. The
first integer found when scanning from right to left starts the indexing portion of the data field.


If the alphanumeric field to be indexed ends with an alpha character, the data will be scanned, characterby-character, from right to left until a numeric character is encountered. Serialization will take place using
the value of the first number found.


1734


Advanced Techniques

## **Variable Data**


To increase throughput, you can set up a program that uses variable data fields. Then, instead of
formatting the whole label each time a label is printed, the printer will have to format only the changed data
field. To use this capability, you must use the `^MC` and `^FV` commands.

## **Stored Formats**


You can create formats and save them in the printers memory. A stored format can then be recalled and
merged with downloaded data to form a complete label. This process saves data transmission time but not
formatting time.


To create a format, complete these steps:


**1.** Design the label.


**2.** Replace variable data fields with field numbers.


**3.** Allocate space for the size of the field.


**4.** Give the format a name.


**5.** Save the format on the printer to a memory location (R, E, B, A).


You can store multiple formats on the printer, limited only by available memory. If you try to save a format
that would overload memory you can confirm that the format has been successfully stored on the printer
by printing the LIST FORMATS from the front panel, or by using the `^HW` command to return the directory
listing to the host. For details see, ^HW.


If the power is turned off, all stored formats in volatile memory (R:) will be lost.

## **Initialize/Erase Stored Formats**


Stored formats can be selectively erased using the `^ID` command.

## **Download Format Command**


The ^DF (Download Format) command saves the ZPL II format commands as text strings to be later merged
using ^XF with variable data. The format to be stored may contain Field Number (^FN) commands to be
referenced when recalled.


While use of stored formats will reduce transmission time, no formatting time is saved since this command
saves the ZPL II as text strings which need to be formatted at print time.

## **Field Number Command**


The ^FN (Field Number) command is used to number the data fields. This command is used in both Store
Format and Recall Format operations.


When storing a format, the ^FN command is used where you would normally use the ^FD (Field Data)
command. When recalling the stored format, use ^FN in conjunction with the ^FD(Field Data) command.


1735


Advanced Techniques

## **Recall Stored Format Command**


The `^XF` (Recall Format) command recalls a stored format to be merged with variable data. There can be
multiple `^XF` commands and they can be located anywhere in the label format.

When recalling a stored format and merging data utilizing the `^FN` (Field Number) function, the calling
format must contain the ^FN command to merge the data properly.


While the use of stored formats will reduce transmission time, no formatting time is saved because the
format being recalled was saved as text strings that need to be formatted at print time.


These are examples of using the stored format:


Working with Stored Format commands involves designing and saving a stored format, then recalling and
merging the format with some variable data.


The following is an example of how to use the various Stored Format commands. First, enter the following
format and send it to the printer. Notice that no label is printed. (DATA Indicator went On and Off.)


Second, enter the following format and send it to the printer. The label shown will be printed.

## **Control Commands**


Control commands may be sent from the host at any time to elicit an immediate response from the printer.
Control commands may be sent in a group or separately.


A control command is acted upon when received to perform a variety of actions, such as:


          - clearing the memory


          - physical action


          - a combination of the above such as feeding a label and calculating and storing its length.


The basic format for using all of the control commands is:


~(2-letter command)


1736


Advanced Techniques


For example: `~DG`

## **Test and Setup Commands**


The following commands, presented in alphabetical order, are used to test various elements of the printer
and its status.


**Table 45** Test and Setup Commands













|Command|Function|
|---|---|
|`~HM`(Memory Status)|Sending this command to the printer immediately returns a memory status<br>message to host. Use this command whenever you need to know the status<br>of the memory.|
|`~HS`(Host Status)|Sending this command to the printer immediately returns a three-line<br>printer status message to the host. Use this command whenever you need<br>to know the status of the printer.|
|`~JR`(Power On Reset)|This command resets all of the printer’s internal software, performs a<br>power-on self-test, clears the buffer and DRAM, and resets communication<br>parameters and default values.`~JR` performs the same function as a<br>manual power-on reset.|
|`~JN`(Head Test Fatal)|This command resets the printhead element error override, acting as a<br>toggle for`~JO`. The printer then goes into fault status (turns head indicator<br>on steadily) if any subsequent execution of the printing element test<br>detects bad printing elements. This command is only functional on certain<br>printer platforms.|
|`~JO`(Head Test Non-<br>Fatal)|This command overrides a failure of head element status check and allows<br>printing to continue. The override is canceled when the printer is turned<br>off or receives a`~JR` or`~JN` command. The printhead test will not produce<br>an error if the`~JO` override is active. This command is only functional on<br>certain printer platforms.|
|`^JT`(Head Test Interval)|This command lets you change the printhead test interval from 100 to any<br>desired interval. The printer automatically performs an internal printhead<br>element test, which occurs every 100 labels. This takes place during<br>formatting which minimizes a delay in printing. Therefore, the test may be<br>performed while the printer is in PAUSE. This command is only functional<br>on certain printer platforms.|
|`~WC` (Print Configuration<br>Label)|The ~WC command is used to generate a printer configuration label.<br>The printer configuration label contains information about the printer<br>setup, such as sensor type, network ID, ZPL mode, firmware version, and<br>descriptive data on the R:, E:, B:, and A: devices.|
|`~HQ` (Host Query)|The ~HQ command group causes the printer to send information back to<br>the host.|


1737


Advanced Techniques

## **Calibration and Media Feed Commands**


The following commands, presented in alphabetical order, are used to perform various media and ribbon
calibrations and also set the media feed mode for the printer.







|Command|Function|
|---|---|
|`~JC`(Set Media Sensor<br>Calibration)|Forces a label length measurement and recalibrates the media and ribbon<br>sensors.<br>**NOTE:** In continuous mode, only the media and ribbon sensors<br>are recalibrated.|
|`~JG`(Graphing Sensor<br>Calibration)|Forces a label length measurement, recalibrates the media and ribbon<br>sensors, and prints a graph (media sensor profile) of the sensor values.|
|`~JL`(Set Label Length)|Sets the label length. Depending on the size of the label, the printer will<br>feed one or more blank labels.|
|`^MF`(Media Feed)|Dictates what happens to the media at power up and after an error is<br>cleared.|

## **Cancel/Clear Commands**

The following command controls the contents of the Zebra input buffer.

|Command|Function|
|---|---|
|`~JA`(Cancel All)|Cancels all format commands in the buffer. It also cancels any batches that<br>may be printing. The printer stops printing after the current label (if one<br>is printing) is finished printing. All internal buffers are cleared of data. The<br>DATA LED turns off.|


## **Printer Control Commands**


The following commands control various printer operations:





|Command|Function|
|---|---|
|`^PF`(Slew Given Number<br>of Dot Rows)|Causes the printer to slew labels (move labels at a high speed without<br>printing) a specified number of dot rows, at the bottom of the label. This<br>allows faster printing when the bottom portion of a label is blank.|
|`~PH` or`^PH`(Slew to<br>Home Position)|Causes the printer to feed one blank label.<br>•<br>The`~PH` command feeds one label after the format currently being<br>printing is done or when the printer is placed in pause.<br>•<br>The`^PH` command feeds one blank label after the format it is in prints.|
|`~PP`(Programmable<br>Pause)|Stops printing after the current label is printed (if one is printing) and places<br>the printer in the Pause mode.The operation is identical to pressing the<br>PAUSE button on the front panel of the printer. The printer will remain<br>paused until the PAUSE button is pressed or a`~PS` command is sent to the<br>printer.|


1738


Advanced Techniques







|Command|Function|
|---|---|
|`^PP`(Programmable<br>Pause)|This command pauses the printer after the format it is in prints. Because<br>this command is not executed immediately, several labels may be printed<br>before the printer is paused.The operation is identical to pressing the<br>PAUSE button on the front panel of the printer. The printer will remain<br>paused until the PAUSE button is pressed or a`~PS` command is sent to the<br>printer.|
|`^PQ`(Print Quantity)|This command gives control over several printing operations. It controls the<br>number of labels to print, the number of labels printed before the printer<br>pauses, and the number of replications of each serial number.|
|`^PR`(Print Rate)|Determines the media speed during printing and the slew speed (feeding<br>a blank label). The printer will operate with the selected speeds until the<br>setting is resent in a subsequent format or the printer is turned off.<br>**Limitations of Higher Print Speeds**<br>Print speed is application specific. Because print quality is affected by<br>media and ribbon, printing speeds, and printer operating modes, it is very<br>important to run tests for your applications.<br>•<br>With high print speeds, use thermal transfer mode only.<br>•<br>Horizontal bar codes with a minimum x dimension of 5 mil may be<br>printed at print speeds of 2 in. (51mm) per second.<br>•<br>Rotated bar codes are limited to a minimum x dimension of 10 mil<br>(modulus 2) at higher print speeds. At x dimension of 5 mil (modulus 1),<br>they may be printed at 2 in. per second.<br>•<br>At high print speeds, Font A at a magnification of 1 is not recommended;<br>all other fonts are acceptable.|
|`~PS`(Print Start)|Causes a printer in the Pause mode to resume printing. The operation is<br>identical to pressing the PAUSE button on the front panel of the printer<br>when the printer is already in the Pause mode.|

## **Set Dots/Millimeter**

The following commands change the number of dots per millimeter.

|Command|Function|
|---|---|
|`^JM`(Set Dots/Millimeter)|Changes the number of dots printed per millimeter. Depending on the<br>printhead, normal dots per millimeter on a Zebra printer are the following:<br>•<br>24 dots/mm (609.6 dots/inch)<br>•<br>12 dots/mm (304.8 dots/inch)<br>•<br>8 dots/mm (203.2 dots/inch)<br>•<br>6 dots/mm (152.4 dots/inch)<br>In some applications, these high densities are not required. For these<br>applications, a lower density of 4 dots/mm (102 dots/inch) or 3 dots/mm (77<br>dots/inch) can be selected.If used, this command must be entered before<br>the first`^FS` command.|



1739


Advanced Techniques

## **Host Status Commands**


The following commands control the host device.


**Table 46** Host Status Commands






|Command|Function|
|---|---|
|`~HI`(Host Identification)|This command is designed to be sent from the Host to the Zebra printer<br>to find out the type of Zebra printer. Upon receipt, the Zebra printer will<br>respond to the Host with a character string that gives information about the<br>printer such as the version of firmware, dots per inch, memory, and printer<br>options.|
|`^SP`(Start Print)|This command allows a label to start printing at a specified point before the<br>entire label has been completely formatted. On extremely complex labels,<br>this command can increase the overall throughput of the printer.<br>The command works as follows: you specify the dot row at which the`^SP`<br>command is to take affect. This then creates a label ‘segment.’ Once the<br>`^SP` command is processed, all information in that segment will be printed.<br>During the printing process, all of the commands after the`^SP` will continue<br>to be received and processed by the printer.<br>If the segment after the`^SP` command (or the remainder of the label) is<br>ready for printing, media motion does not stop. If the next segment is not<br>ready, the printer will stop “mid-label” and wait for the next segment to<br>be completed. Precise positioning of the`^SP` command is somewhat of<br>a trial-and-error process as it depends primarily on print speed and label<br>complexity.<br>The`^SP` command can be effectively used to determine the worst-case<br>print quality. You can determine if using the`^SP` command is appropriate<br>for the particular application by using the following procedure. If you send<br>the label format up to the first ^SP command and then wait for printing to<br>stop before sending the next segment, the printed label will be a sample of<br>the worst case print quality. It will also drop any field that is out of order.|
|`~WC`(Print Configuration<br>Label)|This command is used to generate a printer configuration label. This<br>command only works when the printer is idle.|
|`~WL` Print Network<br>Configuration Label|This command is used to generate a network configuration label. This<br>command only works when the printer is idle.|


## **Changing Delimiters and Command Prefixes**

For some applications, you may need to change the ZPL II default delimiter (,) the format command default
prefix (^), and/or the control command default prefix (~). Any ASCII character may be set as the delimiter.


**IMPORTANT:** The delimiters used in the incoming ZPL script must match the delimiters set
on the printer. If you change the delimiters on the printer, any ZPL script that uses the default
delimiters will not work.


You might change these characters if you are using a hand-held terminal that does not have a comma to
enter the ZPL II commands, if you are working with a mainframe that has trouble processing the caret, or if
you find some other character(s) easier to use.


1740




Advanced Techniques


Reasons to set an alternate delimiter include, but are not limited to:


          - you are using a hand-held terminal that does not have a comma to enter the ZPL II commands;


          - you are working with a host system that does not easily output the default delimiter (for example,
AS/400)


          - you find some other character(s) easier to use.

## **Communication Diagnostics Commands**


Zebra printers support communication diagnostics through both hardware and software control. You can
use these diagnostics to troubleshoot programs.







|Command|Function|
|---|---|
|`~JD`(Enable<br>Communications<br>Diagnostics)|Initiates a diagnostic mode that produces an ASCII printout (using current<br>label length and full width of printer) of all characters received by the<br>printer. This printout includes the ASCII Characters, the HEX value and any<br>communication errors.|
|`~JE`(Disable Diagnostics)|Cancels the diagnostic mode and returns the printer to normal label<br>printing.|

## **Graphic Commands**

In addition to text and bar codes, multiple types of graphics can be printed on a Zebra printer:


          - boxes and lines (^GB), circles (^GC), diagonal lines (^GD), and ellipses (^GE)


          - ZPL II label formats saved as graphics images


          - graphic images in Hexadecimal format


          - graphic symbols (^GS)


**Table 47** Boxes, Lines, Circles, Diagonal Lines, and Ellipses







|Command|Function|
|---|---|
|^GB (Graphic Box)|The ^GB command is used to draw boxes and lines as part of a label format.<br>Boxes and lines are used to highlight important information, divide labels<br>into distinct areas, or to improve the appearance of the label. The same<br>format command is used for drawing either boxes or lines.|
|^GC (Graphic Circle)|The ^GC command produces a circle on the printed label. The command<br>parameters specify the diameter (width) of the circle, outline thickness, and<br>color. Thickness extends inward from the outline.|
|^GD (Graphic Diagonal<br>Line)|The ^GC command produces a circle on the printed label. The command<br>parameters specify the diameter (width) of the circle, outline thickness, and<br>color. Thickness extends inward from the outline.|
|^GE (Graphic Ellipse)|The ^GE command produces an ellipse in the label format.|
|^GS (Graphic Symbol)|The ^GS command enables you to generate the registered trademark,<br>copyright symbol, and other symbols.|


1741


Advanced Techniques


These label formats can also be stored as graphic images and data can be merged with them at print time.
Additionally, ZPL II will permit the printing of graphic images from other sources that have been created
in (or converted to) hexadecimal (HEX) format. Such graphic images can come from a variety of sources,
including CAD programs, draw and paint programs, and scanned images.

## **Image Move**


The `^IM` (Image Move) command performs a direct move of an image from a storage area into the bitmap.
The command is identical to the Recall Graphic command except that there are no sizing parameters.

### **Working with Label Formats as Graphics**


The `^IS` (Image Save) and `^IL` (Image Load) commands are used to save a ZPL label format (including text
and/or bar codes) in the printer’s DRAM, FLASH, or PCMCIA as a special graphic image.

The `^IS` (Image Save) and `^IL` (Image Load) commands are used to save a ZPL label format (including text
and/or bar codes) in the printer’s DRAM, FLASH, PCMCIA, or battery backed up SRAM, as a special graphic
image. This increases the throughput of a series of similar but not identical labels.


Instead of formatting each individual label completely, store the constant fields as an image (known as
creating a template). Then, in subsequent label formats, commands are issued to recall that graphic image
format and merge it with variable data.

## **Working with Hex Graphic Images**


ZPL II can be used to save graphic images in HEX format in DRAM, FLASH, or PCMCIA, depending on the
type of memory installed in your printer.


ZPL II can be used to save graphic images in HEX format in DRAM, FLASH, PCMCIA, or battery backed
up SRAM, depending on the type of memory installed in your printer. The image might be created using
a CAD program, a draw or paint program, or a scanner. These images can then be printed on the label.
Graphic images may be created using a program that creates files in the .PCX format. These files must then
be converted to ZPL II graphic format .GRF (pure hexadecimal data without headers or other extraneous
information) for use as part of a label format.


You can use ZebraDesigner or ZebraNet Bridge Enterprise to convert the .PCX graphic format into the
pure hexadecimal .GRF graphic format. Hexadecimal data may also be directly input as part of a ZPL II
program. Manually preparing a string of HEX code is possible but usually impractical.

## **Alternative Data Compression Scheme for ~DG and ~DB Commands**


There is an alternative data compression scheme recognized by the Zebra printer. This scheme further
reduces the actual number of data bytes and the amount of time required to download graphic images and
bitmapped fonts with the `~DG` and `~DB` commands.

The following represent the repeat counts 1, 2, 3, 4, 5, ...., 19 on a subsequent Hexadecimal value. Values
start with G since 0 through 9 and A through F are already used for HEX values.)


1742


Advanced Techniques


These numbers represent the repeat counts 20, 40, 60, 80,....400 on a subsequent hexadecimal value.


Sending `M6` to the printer is identical to sending the following hexadecimal data:

```
       6666666

```

The M has the value of 7. Therefore `M6` sends seven (7) hexadecimal 6’s.

Sending `hB` to the printer is identical to sending the following hexadecimal data:

```
       BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB

```

The h has a value of 40. Therefore, `hB` sends 40 Hexadecimal B’s.

Repeat Values


Several repeat values can be used together to achieve any desired value.


Several repeat values can be used together to achieve any value desired. vMB or MvB will send 327
hexadecimal B’s to the printer.


          - a comma (,) fills the line, to the right, with zeros (0) until the specified line byte is filled.


          - an exclamation mark (!) fills the line, to the right, with ones (1) until the specified line byte is filled.


          - a colon (:) denotes repetition of the previous line.

## **Recalling a Hexadecimal Graphic Image**


The `^XG` (Recall Graphic) command is used to recall one or more graphic images for printing. This
command is used in a label format to merge pictures such as company logos and piece parts, with text
data to form a complete label.


An image may be recalled and resized as many times per format as needed. Other images and data may
be added to the format.

## **Reducing Download Time of Graphic Images**


There is a method of reducing the actual number of data bytes sent to the printer when using the `~DG`
command.


If the HEX string ends in an even number of zeros (0’s), a single comma (,) can be substituted for ALL of
the zeros. If the HEX string ends in an odd number of zeros, one zero and a single comma is required. The
exclamation mark (!) and the colon (:) described under Repeat Values can also be used.


1743


Advanced Techniques


**NOTE:** The text rows in your editor may not be the same as the dot rows used by ZPL II. The
editor may word wrap or truncate the dot rows. ZPL II ignores the end of a text line (carriage
returns and line feed characters).

## **Transferring Object Between Storage Devices**


The ^TO (Transfer Object) command is used to copy an object or group of objects from one storage device
to another. It is quite similar to the copy function used in personal computers.


Source and destination devices must be supplied and must be different and valid for the action specified.
Invalid parameters will cause the command to be ignored.


There are no defaults associated with this command. However, the asterisk (*) may be used as a wild card
for Object names and extensions. For instance, ZEBRA.* or *.GRF would be acceptable forms for use with
`^TO` command.

The Asterisk (*) can be used to transfer multiple object files (except *.FNT) from the DRAM to the Memory
Card. For example, you have several object files that contain logos. These files are named `LOGO1.GRF`,
`LOGO2.GRF`, and `LOGO3.GRF` .

You want to transfer all of these files to the Memory Card using the name NEW instead of LOGO. By
placing an Asterisk (*) after both LOGO and NEW in the transfer command, you can copy all of these files
with one command. The format for this would be as follows:

```
       ^XA
       ^TOR:LOGO*.GRF,B:NEW*.GRF
       ^XZ

```

**NOTE:** If, during a multiple transfer, a file is too big to be stored on the Memory Card, it will
be skipped. All remaining files will be checked to see if they can be stored. Those that can be
stored, will be stored.

## **Deleting Graphics from Memory**


The `^ID` (Item Delete) command deletes objects, images, fonts, and formats from storage areas selectively
or in groups. This command can be used within a printing format to delete objects just prior to saving new
ones or can be in a stand-alone type format simply to delete objects.


The object name and extension support the use of the asterisk (*) as a wildcard. This allows for easy
deletion of selected groups of objects.

The following are various examples of using the `^ID` command.

To delete just stored formats from DRAM:

```
       ^XA^IDR:*.ZPL^XZ

```

To delete formats and images named SAMPLE from DRAM regardless of the extension:

```
       ^XA^IDR:SAMPLE.*^XZ

```

To delete the image SAMPLE1.GRF prior to storing SAMPLE2.GRF:


1744


Advanced Techniques

```
       ^XA
       ^FO25,25^AD,18,10^FDDelete^FS
       ^FO25,45^AD,18,10^FDthen Save^FS
       ^IDR:SAMPLE1.GRF^FS
       ^ISR:SAMPLE2.GRF^FS
       ^XZ

```

To delete everything from DRAM:

```
       ^XA^IDR:*.*^XZ

## **Defining and Using the AUTOEXEC.ZPL Function**

```

An `AUTOEXEC.ZPL` file function is supported by the printer. It functions in much the same way as the
`AUTOEXEC.BAT` file in MS-DOS.

The `AUTOEXEC.ZPL` file function can be used for setting up various parameters at the time the printer is
powered up (such as `^COY`, `^LL`, `^CWf` ). The function can also be recalled at any time after power up.

This file must initially be in the extra EPROM, FLASH, or PCMCIA memory. When the printer is powered on,
it looks to the extra memory site for the stored format called `AUTOEXEC.ZPL` . If found, the contents of the
file are automatically executed as a stored format.

This is an example of an `autoexe.zpl` file:

```
       ^XA
       ^DFE:AUTOEXEC.ZPL^FS
       ^SEE:JIS.DAT^FS
       ^CW1,E:ANMDJ.TTF^FS
       ^XZ

## **Memory, Flash Cards, and Font Cards**

```

Zebra printers come with a variety of memory device, including DRAM, EPROM, PCMCIA, Flash, socket
Flash, and battery backed-up RAM.


**NOTE:** Not all memory options are available on all printers.


Most Zebra printers allow you to print a printer configuration label, which will show the letter designation
assigned to your printer memory options. For printer models that do not support this feature, use the table
below to see how the memory IDs are assigned. Memory IDs default to these values when the printer is
reset to factory defaults.


**Table 48** Letter Designations for Different Memory Options

|Memory Option|Default Letter Designation|
|---|---|
|EPROM|E:|
|PCMCIA|B:|
|Flash|E:|



1745


Advanced Techniques


**Table 48** Letter Designations for Different Memory Options (Continued)

|Memory Option|Default Letter Designation|
|---|---|
|DRAM|R:|
|Battery backed-up RAM|B: or E:|
|Socket Flash|B:|
|Compact Flash|A:|



A few ZPL II commands directly affect the types of memory available to Zebra printers. These commands
are `~JB`, `^JB` and `~HM`


**Table 49** Commands that Affect Available Memory Types







|Command|Function|
|---|---|
|`~JB` (Reset Battery Dead)|This command is sent to the printer if either of these conditions exist:<br>•<br>If the B: memory card is intentionally cleared (reinitialized).<br>•<br>If the battery supplying power to the Battery Powered Font Card fails<br>and is replaced. (A bad battery would show a “battery dead” condition<br>on the printer configuration label.)<br>**NOTE:** If you replace the battery but do not send this command to<br>the printer, the Battery Powered Font Card will not function.|
|`^JB` (Initialize Flash<br>Memory)|This command is used to initialize the two types of Flash Memory available<br>in the Zebra printers.<br>**NOTE:** Link-OS printers use an automatic memory management<br>system that eliminates the need to manually initialize the Flash<br>Memory system.|
|`~HM` (Host Memory Status)|Sending this command to the printer immediately returns a memory status<br>message to the host. Use this command whenever you need to know the<br>status of the memory. When the Host Memory Status Command,`~HM`, is<br>sent to the Zebra printer, a line of data containing three numbers is sent<br>back to the Host. The following is an example:<br>`1024,0780,0780`<br>•<br>The first value is the total amount of RAM (Random Access Memory)<br>installed in the printer. This number is in Kilobytes.<br>•<br>The second value is the maximum amount of RAM available to the user.<br>This number is in Kilobytes.<br>•<br>The third value is the amount of RAM currently available to the user. This<br>number is in Kilobytes.|


1746


Advanced Techniques

## **Shortcuts and Alternate Schemes for Writing ZPL II Scripts**


ZPL II programming scripts can be written in a variety of ways. There are, however, more efficient ways to
write a ZPL II script depending on the application and the commands used. The following are certain ways
to write the same ZPL II script, each yielding the same result.


The Code 39 bar code shows the ZPL II script written like this:

```
       ^XA^FO100,75^BY3
       ^B3N,N,100,Y,N
       ^FD123ABC^XZ

```

Since it is only one field, however, the entire command can be written as a one line entry:

```
       ^XA^FO100,75^BY3^B3N,N,100,Y,N^FD123ABC^XZ

```

Finally, this script can be further simplified by writing it on one line, using the comma (,) delimiter to reduce
the default parameters in the `^B3` command and eliminating the default parameters at the end of the `^B3`
command:

```
       ^XA^FO100,75^BY3^B3,,100^FD123ABC^XZ

```

You might write your ZPL II scripts in any way that makes sense to you. Some programmers prefer to write
out each format command and field on a line by line basis like this:

```
       ^XA
       ^PR2^FS
       ^LL935^FS
       ^LH30,30^FS
       ^FO20,10^AF^FDZEBRA^FS
       ^FO20,60^B3,,40^FDAA001^FS
       ^FO20,180^AF^SNSERIAL NUMBER 00000000111,1,Y^FS
       ^PQ10^FS
       ^XZ

```

Although this script will print with no problems, it contains unnecessary `^FS` (Field Separator) commands
which have been placed after the format commands. Some programmers feel it is required to place a
`^FS` command at the end of each line, but the `^FS` command is only needed to separate specific fields.
Therefore, the script would transmit more quickly written like this:

```
       ^XA
       ^PR2
       ^LL935
       ^LH30,30
       ^FO20,10^AF^FDZEBRA^FS
       ^FO20,60^B3,,40^FDAA001^FS
       ^FO20,180^AF^SNSERIAL NUMBER 00000000111,1,Y^FS
       ^PQ10
       ^XZ

```

1747


Advanced Techniques


Other programmers prefer to keep the format commands on one line as an organizational preference, like
this:

```
     ^XA^PR2^LL935^LH30,30
     ^FO20,10^AF^FDZEBRA^FS
     ^FO20,60^B3,,40^FDAA001^FS
     ^FO20,180^AF^SNSERIAL NUMBER 00000000111,1,Y^FS
     ^PQ10^XZ

```

The label will print out the same so you should develop a scripting pattern that suits your own
organizational style but one which is efficient and is concerned with keeping transmission times to a
minimum.

### **Font Shortcuts**


There are times when you might include a specific font into your script and use it repeatedly within
different fields.


The following is an example of one way to write this script:

```
     ^XA
     ^FO120,108^A0N,89^FDA Guide to^FS
     ^FO120,207^A0N,89^FDZPL II^FS
     ^FO120,306^A0N,89^FDProgramming^FS
     ^FO120,405^A0N,89^FDLanguage^FS
     ^XZ

```

Notice that the `^FS` command is used on the second to last line to close the field. Actually, it is
unnecessary because the `^XZ` will accomplish the same thing, so we can remove it from our script. Also,
since the font and font size are not changing within the fields, this script can be simplified for quicker
transmission by removing the unnecessary font entries and listing the font information once using the `^CF`
command (see ^CF):

```
     ^XA
     ^CF0,89
     ^FO120,108^FDA Guide to^FS
     ^FO120,207^FDZPL II^FS
     ^FO120,306^FDProgramming^FS
     ^FO120,405^FDLanguage
     ^XZ

```

This script can be made even more efficient by including the `^FB` command to identify the left origin of the
text which occurs at the same place each time. For details, see ^FB:

```
     ^XA
     ^CF0,89
     ^FO120,108
     ^FB800,6
     ^FDA Guide to\&ZPL II\&Programming\&Language
     ^XZ

```

1748


Advanced Techniques


**NOTE:** The entries “\&” within the text indicate a carriage return/line feed as allowed by the `^FB`
command. For details, see ^FB.


If you wanted to change the font type or size within the script, however, you would need to include the
specific font parameters within the field where the change occurs. In this case, you would not want to use
the `^FB` command because the change in font size (in our example below) will affect the y-axis (up and
down) position of the text.

You can still use the `^CF` command, but you will need to include the specific font information on the line
where the change in the field occurs:

```
^XA
^CF0,89
^FO120,108^FDA Guide to^FS
^FO120,207^FDZPL II^FS
^FO120,306^A0N,110^FDProgramming^FS
^FO120,426^FDLanguage
^XZ

```

1749
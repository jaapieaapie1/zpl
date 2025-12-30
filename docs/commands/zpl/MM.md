# ^MM




ZPL Commands


The `^MM` command determines the action the printer takes after a label or group of labels has printed.


**Print Mode**


**NOTE:** Refer to the User Guide for your printer to determine which print modes are supported by
your printer.

**Format:** `^MMa,b`

|Parameters|Details|
|---|---|
|`a =` desired mode|**Values:**<br>`T =` Tear-offa<br>`P =` Peel-off (not available on**S**-300)a<br>`R =` Rewind (depends on printer model)<br>`A =` Applicator (depends on printer model)a<br>`C =` Cutter (depends on printer model)<br>`D =` Delayed cuttera<br>`F =` RFIDa<br>`L =` Reserveda, b<br>`U =` Reserveda, b<br>`K =` Kioskc<br>**Default:** The values available for parameter`a` depend on the printer being used<br>and whether it supports the option.<br>For RFID printers:`A =` R110PAX4 print engines`F =` other RFID printers|
|`b =` prepeel select|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`<br>The command is ignored if parameters are missing or invalid. The current value<br>of the command remains unchanged. This option is not supported by Link-OS<br>printers.|
|a. This value is supported on the KR03 or ZD500R printer.<br>b. This value is supported only on the ZM400 and ZM 600 printers.<br>c. This value is supported only on the KR403 printer.|a. This value is supported on the KR03 or ZD500R printer.<br>b. This value is supported only on the ZM400 and ZM 600 printers.<br>c. This value is supported only on the KR403 printer.|



This list identifies the different modes of operation:


- Tear-off — after printing, the label advances so the web is over the tear bar. The label, with liner
attached, can be torn off manually.


305


ZPL Commands


- Peel-off — after printing, the label moves forward and activates a Label Available Sensor. Printing stops
until the label is manually removed from the printer.


  - Power Peel – liner automatically rewinds using an optional internal rewind spindle.


  - Value Peel – liner feeds down the front of the printer and is manually removed.


  - Prepeel – after each label is manually removed, the printer feeds the next label forward to prepeel
a small portion of the label away from the liner material. The printer then backfeeds and prints the
label. The prepeel feature assists in the proper peel operation of some media types.


- Rewind — the label and liner are rewound on an (optional) external rewind device. The next label is
positioned under the printhead (no backfeed motion).


- Applicator — when used with an application device, the label move far enough forward to be removed
by the applicator and applied to an item. This applies only to printers that have applicator ports and that
are being used in a print-and-apply system.


- Cutter — after printing, the media feeds forward and is automatically cut into predetermined lengths.


- Delayed cutter — When the printer is in the Delayed Cut PRINT MODE, it will cut the label when it
receives the `~JK` (Delayed Cut) command. To activate the `~JK` command, the printer's PRINT MODE
must be set to Delayed Cut and there must be a label waiting to be cut. When the printer is not in the
Delayed Cut PRINT MODE, the printer will not cut the label when it receives the `~JK` command.


**NOTE:** Send `~JK` in a separate file - it cannot be sent at the end of a set of commands.


The Delayed Cut feature can be activated:


- through PRINT MODE on the printer’s control panel

- with a `^MMD` command

- RFID — increases throughput time when printing batches of RFID labels by eliminating backfeed
between labels.


- Kiosk — after printing, the media is moved in a presentation position, most applications maintain a loop
of media in the printer.


**Comments:** Be sure to select the appropriate value for the print mode being used to avoid unexpected
results.


This command is ignored on the HC100™ printer.


306
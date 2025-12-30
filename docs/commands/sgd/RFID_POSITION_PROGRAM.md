# rfid.position.program



This command sets the read/write position of the RFID tag (programming position).


**IMPORTANT:** If this command is used to specify a value for the programming position, this value
will be used for the programming position for all labels until a new position is specified or until
the tag calibration procedure is run.


**Setvar**


This command instructs the printer to set the read/write position of the RFID tag.

```
       ! U1 setvar "rfid.position.program" "value"

```

**Values**

              - `"F0"` to `"Fxxx"` (where `xxx` is the label length in millimeters or `"999"`, whichever is less)

The printer prints the first part of a label until it reaches the specified distance and then begins
programming. After programming, the printer prints the remainder of the label.

              - `"B0"` to `"B30"`

The printer backfeeds the label for the specified distance and then begins programming. To
account for the backfeed, allow empty media liner to extend out of the front of the printer when
using a backward programming position.

              - `"up"` move to the next value

              - `"down"` move to the previous value

**Absolute Mode (all firmware versions)**

              - `xxxx=0` to label length (in dot rows). Move the media to the specified position xxxx on the label,
measured in dot rows from the label top, before encoding. Set to 0 (no movement) if the tag is
already in the effective area without moving the media.


**Relative Mode (firmware versions V53.17.6 and later)**

              - `"F0"` to `"Fxxx"` (where xxx is the label length in millimeters or `999`, whichever is less)

The printer prints the first part of a label until it reaches the specified distance and then begins
programming. After programming, the printer prints the remainder of the label.

              - `"B0"` to `"B30"` (Does not apply to the RP4T printer.)

The printer backfeeds the label for the specified distance and then begins programming. To
account for the backfeed, allow empty media liner to extend out of the front of the printer when
using a backward programming position.


**Default**

              - For printers using V53.17.7 and later: `"F0"` (which moves the leading edge of the label to the
print line)

              - For the R2844-Z and RPAX: `"0"` (no movement)

              - For all other printers or firmware: label length minus 1 mm (1/16 in.)


**Getvar**


This command instructs the printer to respond with the current programming position.


1514


SGD RFID Commands

```
! U1 getvar "rfid.position.program"

```

**Example**


This example shows the programming position being set at 15 mm from the leading edge of the label.

```
! U1 setvar "rfid.position.program" "F15"

```

When the `setvar` value is set to `"F15"`, the `getvar` result is `"F15"` .


1515


SGD RFID Commands
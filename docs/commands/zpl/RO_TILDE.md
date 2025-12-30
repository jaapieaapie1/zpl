# ~RO




ZPL Commands


The `~RO` command resets the advanced counters used by the printer to monitor label generation in inches,
centimeters, and number of labels.


**Reset Advanced Counters**

**Format:** `~ROc`

|Parameters|Details|
|---|---|
|`c =` counter number|**Values:**<br>`1` = reset counter 1<br>`2` = reset counter 2<br>`3` = reset valid RFID label counter<br>`4` = reset voided RFID label counter<br>`C =` reset head cleaned counter1<br>`R =` reset head replaced counter and head cleaned counter1<br>**Default:** a value must be specified or the command is ignored|



1. These values are supported only on Xi4, RXi4, ZM400/ZM600, RZ400/RZ600, S4M, and G-Series
printers.


This example shows how the counter portion of the printer configuration labels looks when counter 1 is
reset by sending `~RO1` .


**Example:** This example shows how the counter portion of the printer configuration labels looks when the
RFID counters are reset by sending `~RO3` and `~RO4` .


329


ZPL Commands


330
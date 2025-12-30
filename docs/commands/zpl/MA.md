# ^MA




ZPL Commands


The `^MA` command controls how the printer issues printed maintenance alerts. Maintenance alerts are
labels that print with a warning that indicates the printhead needs to be cleaned or changed.


**Set Maintenance Alerts**


This command is available only for printers with firmware version V60.15.x, V50.15.x, or later.


- Xi4, RXi4


- ZM400/ZM600, RZ400/RZ600


- S4M with v53.15.5Z or later


- G-Series


**IMPORTANT:** `^MA` settings do not impact or affect the functionality of the Xi4 Supplies Warning
system.

**Format:** `^MAtype,print,printlabel_threshold,frequency,units`





|Parameters|Details|
|---|---|
|`type =` type of alert|**Values:**<br>•<br>`R =` head replacement<br>•<br>`C =` head cleaning<br>**Default:** This parameter must be specified as R or C for`print`,<br>`printlabel_threshold`, and`frequency` to be saved. However,`units`<br>will always be set.|
|`print =` determines if<br>the alert prints a label|**Values:**<br>•<br>`Y =` print a label<br>•<br>`N =` do not print a label<br>**Default:** `N`|
|`printlabel`<br>`threshold=` the<br>distance where the<br>first alert occurs|**Values:**<br>•<br>`R =` head replacement (unit of measurement for head is km with a range of<br>0 to 150 km)<br>•<br>`C =` clean head with a range of 100 to 2000 meters.<br>•<br>`0 =` off (when set to 0, the selected alert is disabled; otherwise, it is<br>enabled.<br>**Default:** `R =` 50 km (1,968,500 inches) and`C =` 0 (off).|
|`frequency =`<br>distance before<br>reissuing the alert|The unit of measurement is in meters. The range is 0 to 2000. The range for<br>G-Series printers is 0 or 5 to 2000 meters. When set to`0`, the alert label is only<br>printed on power-up or when the printer is reset.<br>**Default:** `0` (print on power-up).|


298


ZPL Commands






|Parameters|Details|
|---|---|
|`units =` odometer<br>and printhead<br>maintenance<br>commands|The units parameter reports units of the odometer and printhead maintenance<br>commands, as follows:`~HQOD,~HQPH,~WQOD`, `~WQPH`.<br>**Values:**<br>•<br>`C =` centimeters (displays as:`cm`)<br>•<br>`I =` inches (displays as:`"`)<br>•<br>`M =` meters (displays as:`M`)<br>**Default:** `I`|



**Example:** This example sets the printed head cleaning message to print after five meters and to repeat
every one meter after that until a `~ROC` command is issued.

The Early Warning Maintenance setting must be ON. To enable the maintenance alert system on the GSeries™ printer the `^JH` command is used; on other Zebra printers, the front panel can also be used.

To set `^MA` to print out a label flagging the need to clean the head, type:
```
^XA^MAC,Y,5,1^XZ
```

When the threshold is met, a label will print indicating that the head needs to be cleaned.


For this example, the message on the label looks like this:


For details on resetting the units of measure, see ~HQ examples.


**Comments:**


Any values outside the specified range are ignored.


The intent of this command is to cause a label to print when the defined threshold is reached.


299
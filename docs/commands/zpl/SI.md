# ^SI




ZPL Commands


The `^SI` command is used to change the values for the media sensors, which are also set during the
media calibration process. The media calibration process is described in your specific printer’s user’s
guide.


**Set Sensor Intensity**


This command is available only for printers with firmware versions V53.15.x or later.

**Format:** `^SIa,b`





|Parameters|Details|
|---|---|
|`a =` indicates the setting<br>to modify|**Values:**<br>`1 =` transmissive sensor brightness setting<br>`2 =` transmissive sensor baseline setting<br>**Default:** must be an accepted value or the entire command is ignored|
|`b =` the value to use<br>for the sensor being<br>configured|The ranges for this parameter are the same for the accepted values in<br>parameter`a`.<br>**Values:** `0` to`196`<br>**Default:** must be an accepted value or the entire command is ignored|


337
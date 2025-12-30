# ^SL




ZPL Commands


The `^SL` command is used to specify the Real-Time Clock’s mode of operation and language for printing
information.


**Set Mode and Language (for Real-Time Clock)**

**Format:** `^SLa,b`





|Parameters|Details|
|---|---|
|`a =` mode|**Values:**<br>`S =` Start Time Mode. This is the time that is read from the Real-Time Clock<br>when label formatting begins (when`^XA` is received). The first label has the<br>same time placed on it as the last label.<br>`T =` Time Now Mode. This is the time that is read from the Real-Time Clock<br>when the label to be printed is placed in print queue.**Time Now** is similar to a<br>serialized time or date field.<br>`Numeric Value =` With the Enhanced Real Time Clock (V60.13.0.10 or later)<br>a time accuracy tolerance can be specified. Range = 1 to 999 seconds, 0 = one<br>second tolerance<br>`SL30,1 =` Accuracy tolerance of 30 seconds and use English.<br>**Default:** `S`|
|`b =` language<br>Value 13 is only<br>supported in firmware<br>versions V60.14.x,<br>V50.14.x, or later.|**Values:**<br>`1 =` English<br>`2 =` Spanish<br>`3 =` French<br>`4 =` German<br>`5 =` Italian<br>`6 =` Norwegian<br>`7 =` Portuguese<br>`8 =` Swedish<br>`9 =` Danish<br>`10 =` Spanish 2<br>`11 =` Dutch<br>`12 =` Finnish1<br>`3 =` Japanese<br>`14 =` Korean<br>`15 =` Simplified Chinese<br>`16 =` Traditional Chinese<br>`17 =` Russian<br>`18 =` Polish<br>**Default:** the language selected with`^KL` or the control panel|


338


ZPL Commands


**Comments:** These are some comments to be aware of:


- The **^SL** command must be placed before the first **^FO** command.


- As of V60.13.0.10 all supported printers have Enhanced Real Time Clock capabilities the RTC will not
print time fields that are more than sixty seconds old, rather it will update the time prior to printing
( `^SLT` or `^SL60` ). To control time with increments other than sixty seconds the `^SL` command can be
used with a numeric value ( `^SL30` ). `^SLS` can keep times longer than sixty seconds.

For more details on set mode and language with the Real-Time Clock, see Real Time Clock on page 1595.


339
# ^FC




ZPL Commands


The `^FC` command is used to set the clock indicators (delimiters) and the clock mode for use with the RealTime Clock hardware. This command must be included within each label field command string each time
the Real-Time Clock values are required within the field.


**Field Clock**

**Format:** `^FCa,b,c`






|Parameters|Details|
|---|---|
|`a` = primary clock indicator<br>character|**Values:** any ASCII character<br>**Default:** %|
|b = secondary clock<br>indicator character|**Values:** any ASCII character<br>**Default:** none—this value cannot be the same as`a` or`c`|
|c = third clock indicator<br>character|**Values:** any ASCII character<br>**Default:** none—this value cannot be the same as`a` or`b`|



Entering these ZPL commands sets the primary clock indicator to %, the secondary clock indicator to {, and
the third clock indicator to #. The results are printed on a label with Primary, Secondary, and Third as field
data.


**Comments:** The ^FC command is ignored if the Real Time Clock hardware is not present. As of
V60.13.0.10, ( `^SN` ) functions with ( `^FC` ) capabilities.

For more details on the Real Time Clock, see Real Time Clock on page 1595.


189
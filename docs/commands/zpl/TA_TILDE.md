# ~TA




ZPL Commands


The `~TA` command lets you adjust the rest position of the media after a label is printed, which changes the
position at which the label is torn or cut.


**Tear-off Adjust Position**

**Format:** `~TA###`


**IMPORTANT:** These are some important facts about this command:


- For 600 dpi printers, the step size doubles.


- If the number of characters is **less than** 3, the command is ignored.






|Parameters|Details|
|---|---|
|`### =` change in media<br>rest position (3-digit value<br>in dot rows must be used.)|**Values:**<br>`–120` to`120`<br>`0` to`120` (on the HC100)<br>**Default:** last permanent value saved|



**Comments:** If the parameter is missing or invalid, the command is ignored.


354
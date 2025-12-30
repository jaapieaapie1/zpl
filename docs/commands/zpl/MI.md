# ^MI




ZPL Commands


The `^MI` command controls the content of maintenance alert messages, which are reminders printed by
the printer to instruct the operator to clean or replace the printhead.


**Set Maintenance Information Message**


This command is available only for printers with firmware version V60.15.x, V50.15.x, or later.

**Format:** `^MItype,message`



|Parameters|Details|
|---|---|
|`type =` identifies the<br>type of alert|**Values:**<br>•<br>`R =` head replacement<br>•<br>`C =` head cleaning<br>**Default:** `R`|
|`message =` message<br>that prints on the label<br>when a maintenance<br>alert occurs|The maximum length of each message is 63 characters. All characters<br>following the comma and preceding the next tilde (`~`) or carat`(^`) define the<br>message string. Commas (,) are not allowed in the message.<br>**Default:**<br>•<br>`HEAD CLEANING =` please clean printhead<br>•<br>`HEAD REPLACEMENT =` please replace printhead|


**Example**





This example sets the printhead (head) replacement warning message. Printing of this message is
controlled by the `^MA` command.

1. To customize the text of this label, type something like this:

```
^XA^MIR,PRINT HEAD NEEDS REPLACEMENT - CALL EXT 1000^XZ

```

The label prints whatever you program it to say.


2. For this example, the message on the label looks like this:


303
# ^PP ~PP




ZPL Commands


The `~PP` command stops printing after the current label is complete (if one is printing) and places the
printer in Pause Mode.


**Programmable Pause**

The `^PP` command is not immediate. Therefore, several labels might print before a pause is performed.
This command pauses the printer after the current format prints.


The operation is identical to pressing **PAUSE** on the control panel of the printer. The printer remains
paused until **PAUSE** is pressed or a `~PS` (Print Start) command is sent to the printer.

**Format:** `^PP or ~PP`


322
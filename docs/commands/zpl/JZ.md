# ^JZ




ZPL Commands


The `^JZ` command reprints a partially printed label caused by a Ribbon Out, Media Out, or Head Open
error condition. The label is reprinted as soon as the error condition is corrected.


**Reprint After Error**

This command remains active until another `^JZ` command is sent to the printer or the printer is turned off.

**Format:** `^JZa`

|Parameters|Details|
|---|---|
|`a =` reprint after error|**Values:**<br>`N =` no<br>`Y =` yes<br>**Initial Value at Power Up:** `Y`|



**Comments:** `^JZ` sets the error mode for the printer. If `^JZ` changes, only labels printed after the change
are affected.


If the parameter is missing or incorrect, the command is ignored.


281
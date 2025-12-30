# ^JM




ZPL Commands


The `^JM` command lowers the density of the print—24 dots/mm becomes 12, 12 dots/mm becomes 6, 8
dots/mm becomes 4, and 6 dots/mm becomes 3. `^JM` also affects the field origin ( `^FO` ) placement on the
label (see example below).


**Set Dots per Millimeter**

When sent to the printer, the `^JM` command doubles the format size of the label. Depending on the
printhead, normal dot-per-millimeter capabilities for a Zebra printer are 12 dots/mm (304 dots/inch), 8 dots/
mm (203 dots/inch), or 6 dots/mm (153 dots/inch).

This command must be entered before the first `^FS` command in a format. The effects of `^JM` are
persistent.

**Format:** `^JMn`



|Parameters|Details|
|---|---|
|`n =` set dots per<br>millimeter|**Values:**<br>`A =` 24 dots/mm, 12 dots/mm, 8 dots/mm, or 6 dots/mm<br>`B =` 12 dots/mm, 6 dots/mm, 4 dots/mm, or 3 dots/mm<br>**Default:** `A`|


**Example**





This example shows the effects of alternating the dots per millimeter:


**Comments:** If `^JMB` is used, the UPS MaxiCode bar code becomes out of specification.


269
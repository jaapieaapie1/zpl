# ^PM




ZPL Commands


The `^PM` command prints the entire printable area of the label as a mirror image. This command flips the
image from left to right.


**Printing Mirror Image of Label**

**Format:** `^PMa`






|Parameters|Details|
|---|---|
|`a =` print mirror image of<br>entire label|**Values:**<br>`N =` no<br>`Y =` yes<br>**Default:** `N`|



**Example:** This is an example of printing a mirror image on a label:


**Comments:** If the parameter is missing or invalid, the command is ignored. Once entered, the `^PM`
command remains active until `^PMN` is received or the printer is turned off.


318
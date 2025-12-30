# ^LR




ZPL Commands


The `^LR` command reverses the printing of all fields in the label format. It allows a field to appear as white
over black or black over white.


**Label Reverse Print**

Using the `^LR` is identical to placing an `^FR` command in all current and subsequent fields.

**Format:** `^LRa`

|Parameters|Details|
|---|---|
|`a =` reverse print all fields|**Values:**<br>`N =` no<br>`Y =` yes`N` or last permanently saved value|



**Example**

This is an example that shows printing white over black and black over white. The `^GB` command is used to
create the black background.


**Comments:** The `^LR` setting remains active unless turned off by `^LRN` or the printer is turned off.


**NOTE:** `^GB` needs to be used together with `^LR` .


Only fields following this command are affected.


295
# ^LT




ZPL Commands


The `^LT` command moves the entire label format a maximum of 120 dot rows up or down from its current
position, in relation to the top edge of the label. A negative value moves the format towards the top of the
label; a positive value moves the format away from the top of the label.


**Label Top**


This command can be used to fine-tune the position of the finished label without having to change any of
the existing parameters.


**IMPORTANT:** For some printer models, it is possible to request a negative value large enough
to cause the media to backup into the printer and become unthreaded from the platen. This
condition can result in a printer error or unpredictable results.

**Format:** `^LTx`

|Parameters|Details|
|---|---|
|`x =` label top (in dot rows)|**Values:**<br>HC100:`0` to`120`<br>XiIIIPlus 600dpi:`-240 to 240`<br>All other Zebra printers:`-120` to`120`<br>**Default:** a value must be specified or the command is ignored|



**Comments:** The Accepted Value range for `x` might be smaller depending on the printer platform.

The Label Top value shown on the front panel of the printer is double the value used in the ZPL format.

The `^LT` command does not change the media rest position.


297
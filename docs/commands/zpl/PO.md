# ^PO




ZPL Commands


The `^PO` command inverts the label format 180 degrees. The label appears to be printed upside down. If
the original label contains commands such as `^LL`, `^LS`, `^LT` and `^PF`, the inverted label output is affected
differently.


**Print Orientation**

**Format:** `^POa`



|Parameters|Details|
|---|---|
|`a =` invert the label 180<br>degrees|**Values:**<br>`N =` normal<br>`I =` invert<br>**Default:** `N`|


**Example**





This is an example of printing a label at 180 degrees:


The `^POI` command inverts the x,y coordinates. All image placement is relative to these inverted
coordinates. Therefore, a different `^LH` (Label Home) can be used to move the print back onto the label.

**Comments:** If multiple `^PO` commands are issued in the same label format; only the last command sent to
the printer is used.

Once the `^PO` command is sent, the setting is retained until another `^PO` command is received, or the
printer is turned off.

The `N` value for the `a` parameter is not supported on the HC100™ printer.


321
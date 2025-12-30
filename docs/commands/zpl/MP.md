# ^MP




ZPL Commands


The `^MP` command is used to disable the various mode functions on the control panel. Once disabled,
the settings for the particular mode function can no longer be changed and the LED associated with the
function does not light.


**Mode Protection**

Because this command has only one parameter, each mode must be disabled with an individual `^MP`
command.

**Format:** `^MPa`

|Parameters|Details|
|---|---|
|`a =` mode to protect|**Values:**<br>`D =` disable Darkness Mode<br>`P =` disable Position Mode<br>`C =` disable Calibration Mode<br>`E =` enable all modes<br>`S =` disable all mode saves (modes can be adjusted but values are not<br>saved)<br>`W =` disable Pause<br>`F =` disable Feed<br>`X =` disable Cancel<br>`M =` disable menu changes<br>**Default:** a value must be entered or the command is ignored|



**Example:** This example shows the ZPL code that disables modes D and C. It also shows the effects on the
configuration label before and after the ZPL code is sent:

```
^XA
^MPD
^MPC
^XZ

```

308


ZPL Commands


309
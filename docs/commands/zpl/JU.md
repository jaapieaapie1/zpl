# ^JU




ZPL Commands


The `^JU` command sets the active configuration for the printer.


**Configuration Update**

**Format:** `^JUa`

|Parameters|Details|
|---|---|
|`a =` active configuration|**Values:**<br>`A =`reload factory settings and factory network settings.<br>`F =` reload factory settings.<br>`N =` reload factory network settings.<br>These values are lost at power-off if not saved with`^JUS`.<br>`R =` recall the last saved settings.<br>`S =` save current settings.<br>These values are used at power-on.<br>**Default:** a value must be specified|



278
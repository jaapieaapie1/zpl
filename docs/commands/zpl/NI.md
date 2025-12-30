# ^NI




ZPL Network Commands


The `^NI` command is used to assign a network ID number to the printer. This must be done before the
printer can be used in a network.


**Network ID Number**

**Format:** `^NI###`






|Parameters|Details|
|---|---|
|`### =` network ID<br>number assigned (must<br>be a three-digit entry)|**Values:** `001` to`999`<br>**Default:** `000` (none)|



**Comments:** The last network ID number set is the one recognized by the system.

The commands `~NC`, `^NI`, `~NR`, and `~NT` are used only with RS-485 printer communications.


383
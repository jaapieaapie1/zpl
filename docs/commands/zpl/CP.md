# ^CP




ZPL Commands


The `^CP` command causes the printer to move a printed label out of the presenter area in one of several
ways.


**Remove Label**


**Supported Devices:**


- KR403

**Format:** `^CPa`

|Parameters|Details|
|---|---|
|`a` = kiosk present mode|**Values:**<br>`0` = Eject presented page<br>`1` = Retracts presented page<br>`2` = Takes the action defined by c parameter of ^KV command.<br>**Default:** none The command is ignored if parameters are missing or invalid.|



165
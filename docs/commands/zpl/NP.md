# ^NP




ZPL Network Commands


Use this command to specify whether to use the printer’s or the print server’s LAN/WLAN settings at boot
time. The default is to use the printer’s settings. This setting only applies to the parallel port connect print
servers.


**Set Primary/Secondary Device**


When the printer is set as the primary device, you can set it up using ZPL commands or the Wireless Setup
Wizard utility, and any wired print server inserted into the printer will use those settings. The drawbacks to
using the printer as a primary are:


Any wired print server inserted into the printer will lose its original settings if the printer is set to check for
the wired print server and the Primary Device is set to PRINTER (see ^NB on page 378).

**Format:** `^NPa`





|Parameters|Details|
|---|---|
|`a` = device to use as<br>primary|**Values:**<br>`P =`printer`M =`MPS/Printserver<br>**Default:**`P`|


385
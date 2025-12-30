# ^ND




ZPL Network Commands


The `^ND` command changes the network settings on supported printers.


**Change Network Settings**

For the external wired print server settings, the `^ND` command is the same as the `^NS` command. For the
wireless print server settings, the `^ND` command is the same as the `^WI` command.

**Format:** `^NDa,b,c,d,e,f,g,h,i,j`

















|Parameters|Details|
|---|---|
|`a =` the device that is<br>being modified|**Values:**<br>•<br>`1 =` external wired<br>•<br>`2 =` internal wired<br>•<br>`3 =` wireless|
|`b =` IP resolution|**Values:**<br>•<br>`A` = All<br>•<br>`B` = BOOTP<br>•<br>`C` = DHCP and BOOTP<br>•<br>`D` = DHCP<br>•<br>`G` = Gleaning only (Not recommended when the Wireless Print Server or<br>Wireless Plus Print Server is installed.)<br>•<br>`R` = RARP<br>•<br>`P` = Permanent<br>**Default:** `A`|
|`c =` IP address|**Values:** Any properly formatted IP address in the xxx.xxx.xxx.xxx format.|
|`d =` subnet mask|**Values:** Any properly formatted subnet mask in the xxx.xxx.xxx.xxx format.|
|`e =` default gateway|**Values:** Any properly formatted gateway in the xxx.xxx.xxx.xxx format.|
|`f =` WINS server<br>address|**Values:** Any properly formatted WINS server in the xxx.xxx.xxx.xxx format.|
|`g =` connection<br>timeout checking|**Values:**<br>`Y =` yes<br>`N =` no<br>**Default:** `Y`|
|`h =` timeout value|Time, in seconds, before the connection times out.<br>**Values:** `0` through`9999`<br>**Default:** `300`|
|`i =` ARP broadcast<br>interval|Time, in minutes, that the broadcast is sent to update the device’s ARP cache.<br>**Values:** `0` through`30`<br>**Default:** `0` (no ARP sent)|


381


ZPL Network Commands





|Parameters|Details|
|---|---|
|`j =` base raw port<br>number|The port number that the printer should use for its RAW data.<br>**Values:** `1` through`65535`<br>**Default:** `9100`|


382
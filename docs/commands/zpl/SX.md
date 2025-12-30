# ^SX




ZPL Commands


The `^SX` command is used to configure the ZebraNet Alert System.


**Set ZebraNet Alert**

**Format:** `^SXa,b,c,d,e,f`


**NOTE:** The values in this table apply to firmware version V48.12.4 or later.

|Parameters|Details|
|---|---|
|`a =` condition type|**Values:**<br>`A =` paper out<br>`B =` ribbon out<br>`C =` printhead over-temp<br>`D =` printhead under-temp<br>`E =` head open<br>`F =` power supply over-temp<br>`G =` ribbon-in warning (Direct Thermal Mode)<br>`H =` rewind full<br>`I =` cut error<br>`J =` printer paused<br>`K =` PQ job completed<br>`L =` label ready<br>`M =` head element out<br>`N =` ZBI (Zebra BASIC Interpreter) runtime error<br>`O =` ZBI (Zebra BASIC Interpreter) forced error<br>`P =` power on<br>`Q =` clean printhead<br>`R =` media low<br>`S =` ribbon low<br>`T =` replace head<br>`U =` battery low<br>`V =` RFID error (in RFID printers only)<br>`* =` all errors<br>**Default:** if the parameter is missing or invalid, the command is ignored|



351


ZPL Commands







|Parameters|Details|
|---|---|
|`b =` destination for route<br>alert|**Values:**<br>`A =` serial port<br>`B* =` parallel port<br>`C =` e-mail address<br>`D =` TCP/IP<br>`E =` UDP/IP<br>`F =` SNMP trap<br>**Default:** If this parameter is missing or invalid, the command is ignored.<br>*** Requires** bidirectional communication.|
|`c =` enable condition set<br>alert to this destination|**Values:**<br>`N =` no<br>`Y =` yes<br>**Values:** `Y` or previously configured value|
|`d =` enable condition<br>clear alert to this<br>destination|**Values:**<br>`N =` no<br>`Y =` yes<br>**Values:** `N` or previously configured value<br>Parameters`e` and`f` are sub-options based on destination. If the sub-<br>options are missing or invalid, these parameters are ignored.|
|`e =` destination setting|**Values:**<br>Internet e-mail address (e.g. user@company.com)<br>IP address (for example, 10.1.2.123)<br>SNMP trap<br>IP or IPX addresses|
|`f =` port number|**Values:**<br>TCP port # (`0` to`65535`)<br>UPD port # (`0` to`65535`)|


**Example:** This is an example of the different ( `b` ) destinations that you can send for the condition type ( `a` ):

Serial: `^SXA,A,Y,Y`

Parallel: `^SXA,B,Y,Y`

E-Mail: `^SXA,C,Y,Y,admin@company.com`

TCP: `^SXA,D,Y,Y,123.45.67.89,1234`

UDP: `^SXA,E,Y,Y,123.45.67.89,1234`

SNMP Trap: `^SXA,F,Y,Y,255.255.255.255`

**Comments:** In the example above for SNMP Trap, entering 255.255.255.255 broadcasts the notification
to every SNMP manager on the network. To route the device to a single SNMP manager, enter a specific
address (123.45.67.89).


352
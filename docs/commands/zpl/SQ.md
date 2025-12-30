# ^SQ




ZPL Commands


The `^SQ` command is used to stop the ZebraNet Alert option.


**Halt ZebraNet Alert**

**Format:** `^SQa,b,c`

|Parameters|Details|
|---|---|
|`a =` condition type|**Values:**<br>`A =` paper out<br>`B =` ribbon out<br>`C =` printhead over-temp<br>`D =` printhead under-temp<br>`E =` head open<br>`F =` power supply over-temp<br>`G =` ribbon-in warning (Direct Thermal Mode)<br>`H =` rewind full<br>`I =` cut error<br>`J =` printer paused<br>`K =` PQ job completed<br>`L =` label ready<br>`M =` head element out<br>`N =` ZBI (Zebra BASIC Interpreter) runtime error<br>`O =` ZBI (Zebra BASIC Interpreter) forced error<br>`Q =` clean printhead<br>`R =` media low<br>`S =` ribbon low<br>`T =` replace head<br>`U =` battery low<br>`V =` RFID error (in RFID printers only)<br>`W =` all errors (in RFID printers only)<br>`* =` all errors (in non-RFID printers)|
|`b =` destination|**Values:**<br>`A =` serial port<br>`B =` parallel port<br>`C =` e-mail address<br>`D =` TCP/IP<br>`E =` UDP/IP<br>`F =` SNMP trap<br>`* =` wild card to stop alerts for all destinations|



345


ZPL Commands

|Parameters|Details|
|---|---|
|`c =` halt messages|**Values:**<br>`Y =` halt messages<br>`N =` start messages<br>**Default:** `Y`|



346
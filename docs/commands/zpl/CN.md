# ^CN




ZPL Commands


The ^CN causes the printer to cycle the media cutter.


**Cut Now**


**IMPORTANT:** This command works only when the printer is in Print Mode Kiosk ( `^MMk` ). If the
printer is not in Print Mode Kiosk, then using this command has no effect. See ^MM on page
305.


**Supported Devices:**


- KR403

|Format: ^CNa|Col2|
|---|---|
|Parameters|Details|
|`a` = Cut Mode Override|**Values:**<br>`0` = Use the “kiosk cut amount” setting from`^KV`<br>`1` = Ignore “kiosk cut amount” from`^KV` and do a full cut<br>**Default:** none<br>The command is ignored if parameters are missing or invalid.|



162
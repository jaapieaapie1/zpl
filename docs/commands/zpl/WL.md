# ^WL - Set Leap



Use this command to enable Cisco [®] Lightweight Extensible Authentication Protocol (LEAP) mode and set
parameters. LEAP is user authentication method that is available with some wireless radio cards.


**Set LEAP Parameters**


**NOTE:** The `^WL` command is provided only for backward-compatibility with printers using
firmware prior to V50.15.x or X60.15.x. For these firmware versions and later, use **^WX on page**
**425** to set the security type and related parameters.

**Format:** `^WLa,b,c`

|Parameters|Details|
|---|---|
|`a` = mode|**Values:**OFF, ON<br>**Default:**OFF|
|`b` = user name|**Values:**Any 1 to 32 alphanumeric including special characters<br>**Default:**user|
|`c` = password|**Values:**Any 1 to 32 alphanumeric including special characters<br>**Default:**password|



395


ZPL Network Commands
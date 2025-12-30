# ^WP




ZPL Network Commands


Use this command to set the four-digit wireless password (not the same as the general printer password). If
the wireless password is `0000`, the Wireless and Wireless Plus print servers run in an “unprotected” mode,
which means that you do not need to enter the wireless password through the control panel to view or
modify wireless settings.


**Set Wireless Password**


**NOTE:** This command does not apply to the S4M.


If a wireless password is set, the values for the following parameters will not appear through the control
panel until the wireless password is entered:


- MAC Address


- ESSID


- WLAN Security


- WEP Type


- WEP Index


- Reset Network


**Format:** ^WPa,b





|Parameters|Details|
|---|---|
|`a` = old wireless password|**Values:**0000 through 9999<br>**Default:**0000|
|`b` = new wireless<br>password|**Values:**0000 through 9999<br>**Default:**0000|


398


ZPL Network Commands
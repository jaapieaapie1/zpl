# ^NW




ZPL Network Commands


Use this command to set the timeout value for the printer home page. The printer will prompt for the printer
password only the first time that certain screens are accessed until 1) the web authentication timeout value
is reached (default value is 5 minutes) or 2) the printer is reset. At that time, the printer will prompt for the
password again.


**Set Web Authentication Timeout Value**

**Format:** `^NWa`

|Parameters|Details|
|---|---|
|`a` = timeout value|The timeout value in minutes for an IP address to be authenticated to the<br>printer web pages.<br>**Values:**`0` (no secure pages can be accessed without entering the printer<br>password) to 255 minutes<br>**Default:**`5`|



391
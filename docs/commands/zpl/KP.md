# ^KP




ZPL Commands


The `^KP` command is used to define the password that must be entered to access the control panel
switches and LCD Setup Mode.


**IMPORTANT:** This command does not work when the printer is in Protected
Mode. With Protected Mode, use JSON-formatted Protect commands with the SGD
`display.password.current` instead. The information below assumes that your printer is not
in Protected Mode.


[For more information about the front panel password, refer to the Zebra Link-OS PrintSecure Printer](https://www.zebra.com/content/dam/support-dam/en/documentation/unrestricted/guide/software/printsecure-administration-guide-en.pdf)
[Administration Guide.](https://www.zebra.com/content/dam/support-dam/en/documentation/unrestricted/guide/software/printsecure-administration-guide-en.pdf)


**Define Password**

**Format:** `^KPa,b`






|Parameters|Details|
|---|---|
|`a` = mandatory four-digit<br>password|**Values:** `0000` to`9999` (four digits)<br>**Default:**<br>•<br>**For printers purchased in the EMEA region after August 1, 2025:** The<br>default value is empty and must be set before it can be used on the<br>printer.<br>•<br>**For all other printers:** `1234`|
|`b` = password level<br>(applicable to the S4M<br>printer only)|**Values:** 1, 2, 3, 4<br>**Default:** 3|



**Example:** This example shows how to set a new password of `5678` :

```
^XA
^KP5678
^XZ

```

**Example (applicable to the S4M printer only):** This example shows how to set the front panel password to
`5678` at a specific password level (level 2):

```
^XA
^KP5678,2
^XZ

```

287
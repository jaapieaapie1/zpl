# ~SD




ZPL Commands


The `~SD` command allows you to set the darkness of printing. `~SD` is the equivalent of the darkness setting
parameter on the control panel display.


**Set Darkness**

**Format:** `~SD##`






|Parameters|Details|
|---|---|
|`## =` desired darkness<br>setting (two-digit number)|**Values:** `00` to`30`<br>**Default:** last permanently saved value|



**IMPORTANT:** The darkness setting range for the XiIIIPlus, Xi4, and RXi4 is 0 to 30 in increments
of 0.1. The firmware is setup so that the `^MD` and `~SD` commands (ZPL darkness commands)
accept that range of settings.


**Example:** These are examples of the XiIIIPlus, Xi4, and RXi4 Darkness Setting:

```
^MD8.3
~SD8.3

```

**Comments:** The `^MD` command value, if applicable, is added to the `~SD` command.


332
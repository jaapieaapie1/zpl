# ~PM




ZPL Commands


This command places the printer into the Decommissioning Mode.


**Decommissioning Mode**


**Format:** ~PMa,b

|Parameter|Details|
|---|---|
|`a` = printer’s serial number|Alphanumeric string. Mandatory parameter.|
|`b` = number of times for the flash wipe-out to occur|Optional parameter.<br>Minimum Value = 0<br>Maximum Value =3<br>Default Value = 0|



**NOTE:**


- This command is only accepted via USB. It will be ignored if sent over any other
communication channel such as Bluetooth or Ethernet. This is done as a security measure to
ensure that the person sending the command has physical access to the printer.


- If Decommissioning Mode takes longer than 3 seconds, printers with a screen will display a
message while the Decommissioning Mode is in process.


- The serial number specified should match with the serial number of the printer, else the
printer does not exit Protected Mode.


- Decommissioning Mode will cause the printer to exit Protected Mode.


- Only use the flash wipe optional parameter if the printer will be resold, recycled, or reused
by another group that should not have access to the printer settings data. This may include
proprietary fonts, formats, files, or network configuration. A flash wipe does take considerable
time, which will vary in length, based on printer model.


**RECOMMENDATION:** Only use the flash wipe optional parameter if the printer will be resold, recycled,
or reused by another group that should not have access to the printer settings data. This may include
proprietary fonts, formats, files, or network configuration. A flash wipe takes considerable time, which will
vary in length, based on the printer model.


319
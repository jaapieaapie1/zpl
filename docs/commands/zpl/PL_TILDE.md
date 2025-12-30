# ~PL




ZPL Commands


The ~PL command adds an additional amount to how far the paper is ejected during a present cycle. A
standard amount of 50 mm is always added to clear the kiosk wall. This amount is added to that 50mm.
The total amount of media ejected when a ^PN is executed, then, is 50 mm + ~PL value + ^PN value.


**Present Length Addition**


**Supported Devices:**


- KR403

**Format:** `^PLa`

|Parameters|Details|
|---|---|
|a = additional eject length|**Values:**<br>000-255 = additional mm of media to eject<br>**Default:** 000<br>The command is ignored if parameters are missing or invalid.|



317
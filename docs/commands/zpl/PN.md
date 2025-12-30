# ^PN




ZPL Commands


The ^PN command causes the printer to run a Presenter cycle. The parameter defines the amount of media
ejected. The total amount of media ejected when a ^PN is executed, then, is 50 mm + ~PL value + ^PN
value. (See ~PL).


**Present Now**


**Supported Devices:**


- KR403

**Format:** `^PNa`

|Parameters|Details|
|---|---|
|a = media eject length|**Values:**<br>0-255 = additional mm of media to eject<br>**Default:** none<br>The command is ignored if parameters are missing or invalid.|



320
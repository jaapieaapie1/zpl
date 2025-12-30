# ^JI




ZPL Commands


`^JI` works much like the `~JI` command. Both commands are sent to the printer to initialize the Zebra
BASIC Interpreter.


**Start ZBI (Zebra BASIC Interpreter)**


Identifies features that are available in printers with firmware version V60.16.2Z, V53.16.2Z, or later.

In interactive mode, `^JI` can be sent through one of the communication ports (serial, parallel, or Ethernet)
to initialize the printer to receive ZBI commands. This command can be sent from one of the Zebra
software utilities, such as ZTools, or from a terminal emulation program.


When the command is received, the printer responds by sending a ZBI header back to the console, along
with the program version number. This indicates that the interpreter is active.

**Format:** `^JId:o.x,b,c,d`












|Parameters|Details|
|---|---|
|d = location of<br>program to run after<br>initialization|**Values:** `R:`, `E:`, `B:`, and`A:`<br>**Default:** location must be specified|
|o = name of<br>program to run after<br>initialization|**Values:** any valid program name<br>**Default:** name must be specified|
|x = extension of<br>program to run after<br>initialization|**Fixed Value:** `.BAS`, `.BAE`<br>**NOTE:** `.BAE` is only supported in firmware version V60.16.0Z or later|
|b = console control|**Values:**<br>`Y` = console on<br>`N` = console off<br>**Default:** `Y`|
|c = echoing control|**Values:**<br>`Y` = echo on<br>`N` = echo off<br>**Default:** `Y`|
|d = memory allocation<br>for ZBI *|**Values:** `20K` to`1024K`<br>**Default:** `50K`|




- This parameter is only available on printers with firmware V60.12.0.x or earlier.


**Comments**


When the printer is turned on, it can receive ZPL II commands and label formats. However, for the printer to
recognize ZBI commands and programs, it must be initialized using `^JI` or `~JI` .

Only one ZBI interpreter can be active in the printer at a time. If a second `^JI` or `~JI` command is received
while the interpreter is running, the command is ignored.


263


ZPL Commands


The interpreter is deactivated by entering one of two commands:

`ZPL` at the ZBI prompt

`~JQ` at an active ZPL port


264
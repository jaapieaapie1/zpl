# ~JI




ZPL Commands


`~JI` works much like the `^JI` command. Both commands are sent to the printer to initialize the Zebra
BASIC Interpreter.


**Start ZBI (Zebra BASIC Interpreter)**


Identifies features that are available in printers with firmware version V60.16.2Z, V53.16.2Z, or later.

In interactive mode, `~JI` can be sent through one of the communication ports (serial, parallel, or Ethernet)
to initialize the printer to receive ZBI commands. This command can be sent from one of the Zebra
software utilities, such as ZTools, or from a standard PC program, such as Hyper terminal.


When the command is received, the printer responds by sending a ZBI header back to the console, along
with the program version number. This indicates that the interpreter is active.

**Format:** `~JI`

**Comments:** While receiving commands, the printer echoes the received characters back to the source.
This can be toggled on and off with the ZBI ECHO command.


When the printer is turned on, it can receive ZPL II commands and label formats. However, for the printer to
recognize ZBI commands and formats, it must be initialized using `^JI` or `~JI` .

Only one ZBI interpreter can be active in the printer at a time. If a second `~JI` or `^JI` command is received
while the interpreter is running, the command is ignored.


The interpreter is deactivated by entering one of these commands:

`ZPL` at the ZBI prompt

`~JQ` at an active ZPL port


265
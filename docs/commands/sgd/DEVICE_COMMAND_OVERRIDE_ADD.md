# device.command_override.add



This command adds a specified command to the list of commands that will be ignored by the printer. The
list is saved when the printer is powered off and is not cleared when the printer is defaulted.

          - If there are items in the list and the `device.command_override.active` is set to `"yes"`, then the
config label will show `ACTIVE COMMAND OVERRIDE.`

          - If there are no items in the list or `device.command_override.active` is set to `"no"`, then the
config label will show `INACTIVE COMMAND OVERRIDE.`


**Setvar**


To instruct the printer to add a specified command to the list of override commands:

```
       ! U1 setvar "device.command_override.add" "command"

```

**Values**


Any ZPL or Set/Get/Do command.


**Default**
NA


**Example**


When specifying a ZPL command, the command must be preceded by the current format or control prefix
character (e.g. `^` or `~` ). Multiple commands must be declared with their own `setvar` declaration.

```
       ! U1 setvar "device.command_override.add" "^MN"
       ! U1 setvar "device.command_override.add" "^PR"
       ! U1 setvar "device.command_override.add" "comm.baud"
       ! U1 setvar "device.command_override.add" "device.reset"

```

The following example is not valid.

```
       ! U1 setvar "device.command_override.add" "~HI,~HS,^MN"

```

Instead, send the command as:

```
       ! U1 setvar "device.command_override.add" "~HI"
       ! U1 setvar "device.command_override.add" "~HS"
       ! U1 setvar "device.command_override.add" "^MN"

```

You cannot add `"device.command_override.clear"` to the list of accepted override commands.


677


SGD Printer Commands
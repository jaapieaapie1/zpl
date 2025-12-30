# device.command_override.active



This command enables or disables the `device.command_override` function. When enabled, the printer
will ignore the list of commands previously specified using the `device.command_override.add` . Use
of this command does not modify the list of commands to be overridden.


**NOTE:** This setting is not defaulted as part of a factory default ( `^JUF` or `^default` ). The setting
is persistent across a power cycle or rest ( `~JR` or `device.reset` ).


**Setvar**

To turn on/off the `device.command_override` function:

```
       ! U1 setvar "device.command_override.active" "value"

```

**Values**

              - `"yes"` = active

              - `"no"` = inactive

**Default**
```
          "yes"

```

**Getvar**

To return the active/inactive state of `device.command_override` command:

```
       ! U1 getvar "device.command_override.active"

```

**Result**

              - `"yes"` the command is active

              - `"no"` the command is inactive


**Example**

```
       ! U1 setvar "device.command_override.active" "yes"

       ! U1 setvar "device.command_override.active" "no"

```

676


SGD Printer Commands
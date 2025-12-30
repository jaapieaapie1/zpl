# bluetooth.discoverable



This command enables or disables the Bluetooth discoverable mode.


**Setvar**


To enables or disables the Bluetooth discoverable mode:

```
       ! U1 setvar "bluetooth.discoverable" "value"

```

**Values**

`"on"` enables Bluetooth discoverable mode

`"off"` disables Bluetooth discoverable mode

**Default**

`"on"` for Printers running Link-OS v5.3 or earlier versions

`"off"` for Printers running Link-OS 6 or later versions `)`


**Getvar**


To retrieve the current Bluetooth discoverable mode:

```
       ! U1 getvar "bluetooth.discoverable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "bluetooth.discoverable" "on"

```

What the setvar value is set to is the `getvar` result. In this example, the getvar result is "on".


1096


SGD Network Commands
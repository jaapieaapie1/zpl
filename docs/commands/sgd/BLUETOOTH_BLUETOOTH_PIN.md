# bluetooth.bluetooth_pin



This command is used to connect to the printer only when the command `bluetooth.authentication`
is set to `"setpin"` .


**Setvar**


To set the Bluetooth pin value:

```
       ! U1 setvar "bluetooth.bluetooth_pin" "value"

```

**Values**


Any text string up to 10 characters


**Default**
```
          ""

```

**Getvar**


To retrieves the current Bluetooth pin:

```
       ! U1 getvar "bluetooth.bluetooth_pin"

```

**Example**

This `setvar` example shows the value set to `"1234567890"` .

```
       ! U1 setvar "bluetooth.bluetooth_pin" "1234567890"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is
`"1234567890"` .


1093


SGD Network Commands
# bluetooth.friendly_name



This command sets the friendly name, which is used during service discovery. For changes
to take effect, you must power cycle the printer or issue the `device.reset` command. If
`bluetooth.friendly_name` is not set by you, it will default to the printer serial number.


**Setvar**


To set the Bluetooth discoverable mode:

```
       ! U1 setvar "bluetooth.friendly_name" "value"

```

**Values**


Any text string up to 17 characters


**Getvar**


To retrieve the current Bluetooth discoverable mode:

```
       ! U1 getvar "bluetooth.friendly_name"

```

**Example**

This `setvar` example shows the value set to `"1234567"` .

```
       ! U1 setvar "bluetooth.friendly_name" "1234567"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"1234567"` .


1099


SGD Network Commands
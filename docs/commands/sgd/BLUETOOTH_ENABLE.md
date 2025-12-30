# bluetooth.enable



This command enables or disables the Bluetooth radio.


**Setvar**


To enable or disable the Bluetooth radio:

```
       ! U1 setvar "bluetooth.enable"

```

**Values**

              - `"on"` enables the Bluetooth radio

              - `"off"` disables the Bluetooth radio

**Default**
```
          "on"

```

**Getvar**


To retrieve the current status of the Bluetooth radio:

```
       ! U1 getvar "bluetooth.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "bluetooth.enable" "on"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"on"` .


1097


SGD Network Commands
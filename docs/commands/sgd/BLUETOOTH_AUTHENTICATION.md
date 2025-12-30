# bluetooth.authentication



This command sets or retrieves Bluetooth authentication mode and works in combination with the
`bluetooth.bluetooth_pin` .


**Setvar**


To enable or disable Bluetooth authentication:

```
       ! U1 setvar "bluetooth.authentication" "value"

```

**Values**

              - `"off"` disables authentication (can connect to master device without PIN)

              - `"setpin"` enables authentication (requires PIN or passkey to connect to a master device)

**Default**
```
          "off"

```

**Getvar**


To retrieve the current Bluetooth authentication mode:

```
       ! U1 getvar "bluetooth.authentication"

```

**Example**

This `setvar` example shows the value set to `"setpin"` .

```
       ! U1 setvar "bluetooth.authentication" "setpin"

```

What the `setvar` value is set to is the `getvar` result. In this example, the `getvar` result is `"setpin"` .


1092


SGD Network Commands
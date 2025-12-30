# bluetooth.enable_reconnect



Enables the Bluetooth reconnect feature required by iOS devices.


**Setvar**


To enable or disable the Bluetooth reconnect feature:

```
       ! U1 setvar "bluetooth.enable_reconnect" "value"

```

**Values**

              - `"off"` disables the command

              - `"iOS_only"` enables the command to work with iOS devices only


**Getvar**


To return the current setting value:

```
       ! U1 getvar "bluetooth.enable_reconnect"

```

1098


SGD Network Commands
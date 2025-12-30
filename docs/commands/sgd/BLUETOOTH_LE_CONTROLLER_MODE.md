# bluetooth.le.controller_mode



For printers that support both Bluetooth Classic and Bluetooth Low-Energy mode, this command controls
the mode of operation.


**Setvar**


To control the mode of operation:

```
       ! U1 setvar "bluetooth.le.controller_mode" "value"

```

**Values**

              - `"both"` means the Bluetooth radio operates in both low energy and classic mode

              - `"le"` means the Bluetooth radio operates in low energy mode

              - `"classic"` means the Bluetooth radio operates in the classic mode

**Default**
```
          "both"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "bluetooth.le.controller_mode"

```

1102


SGD Network Commands
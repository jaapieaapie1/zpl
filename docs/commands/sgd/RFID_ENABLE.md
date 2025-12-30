# rfid.enable



This command instructs an RFID printer to enable or disable RFID functionality. You must restart the printer
for the command to take effect.


**NOTE:** When this function is set to `"on"`, changes are made to normal printer functionality.
Loading printer defaults does NOT:


          - Default the sensor select setting


          - Default media tracking sensor settings


          - Default label length


          - Perform an auto calibration


**Setvar**


To enable or disable RFID functionality:

```
       ! U1 setvar "rfid.enable" "value"

```

**Values**

`"on"` enables RFID functionality

`"off"` disables RFID functionality

**Default**
```
          "on"

```

1511


SGD RFID Commands
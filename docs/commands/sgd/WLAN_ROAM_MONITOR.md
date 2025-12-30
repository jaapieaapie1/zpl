# wlan.roam.monitor



This command sets the wireless LAN roam monitoring event messages.


**Setvar**


To instruct the printer to turn off or print the roam event messages:

```
       ! U1 setvar "wlan.roam.monitor" "value"

```

**Values**

`"off"` roam monitor event messages are turned off

`"print"` roam monitor event messages are printed.

`"serial"` roam monitor event messages are output to the serial port.

`"file"` roam monitor event messages are stored in the roam.log file on the E: drive.


**Default**
```
          "off"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.roam.monitor"

```

1473


SGD Network Commands
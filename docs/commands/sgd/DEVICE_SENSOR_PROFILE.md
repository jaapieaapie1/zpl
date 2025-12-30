# device.sensor_profile



This command sets the printer’s sensor profile output destination.


**Setvar**


To set the sensor profile of the printer:

```
       ! U1 setvar "device.sensor_profile" "value"

```

**Values**


If a valid setting is not specified, then the sensor profile command is ignored.


`"print"` forces all subsequent `~jg` output to be printed on media.

`"store"` forces all subsequent `~jg` output to be stored on the E drive.

`"usb_host"` forces all subsequent `~jg` output to be stored on a USB Stick. If the sensor
profile value of "usb_host" is selected and a USB Stick is not inserted at the
time, then the `~jg` is issued. An Acknowledged Alert is displayed on printers
that support an LCD. If the Sensor Profile value of "usb_host" is selected and
a USB Stick is not inserted at the time, then the `~jg` is issued and the printer
does not support an LCD, then the error is ignored.

`"reply"` forces all subsequent `~jg` output to be returned to the host on the same port
that the command is issued.

`"display"` forces all subsequent `~jg` output to be stored on the LCD. If the Sensor
Profile value of "display" is selected and the printer does not have at least
a 240x128 pixel display, then the setting is ignored. The UI SRS defines the
actual content to be displayed if the "display" option is selected.


**Default**
```
          "print"

```

**Getvar**


To return the sensor profile values:

```
       ! U1 getvar "device.sensor_profile"

```

749


SGD Printer Commands
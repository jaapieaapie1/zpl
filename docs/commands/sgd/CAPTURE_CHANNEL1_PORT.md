# capture.channel1.port



This command determines the port that should be monitored for user data. This allows the user to
attach an external device, such as a keyboard or barcode scanner, and have input captured into the
`capture.channel1.data.raw` command. Once the data is in the SGD they can use it as they would
any other SGD (this includes functionality that allows users to be sent an alert when an SGD value
changes).

The data received on the specified port will be read until the value in `capture.channel1.delimiter`
is seen, at which point the data received until (but not including) the delimiter will be stored in
`capture.channel1.data.raw` .

For the port specified in `capture.channel1.port`, no data will be sent to any of the
parsers on that port. All data received is assumed to be user input that is to be placed
in `capture.channel1.data.raw` . To disable the data capture functionality, set
`capture.channel1.port` to `"off"`

The delimiter will not be stored in `capture.channel1.data.raw` . The port will be shown in the data
capture portion of the HZA response. The capture port shall be defaulted to `“off”` by any mechanism
(including `^JUF, ^JUA`, and `device.restore_defaults` ).


**Setvar**


To set the port to be monitored for user data:

```
       ! U1 setvar "capture.channel1.port" "value"

```

**Values**

              - `off` means no data is stored in `capture.channel1.data.raw` and all data is sent to the
parsers - normal operation

              - `serial` means data is read off the serial port. No data sent to the parsers on this port.

              - `usb` means ata is read off the USB port. No data sent to the parsers on this port.

              - `bt` means data is read off the Bluetooth [®] port. No data sent to the parsers on this port.

              - `usb_host` is not yet supported. Reserved for when usb host is implemented.

**Default**
```
          "off"

```

**Getvar**


To retrieve the printer’s current port being monitored for user data:

```
       ! U1 getvar "capture.channel1.port"

```

**Example**

This example sets the command value to `"off"`, preventing it from capturing data.

```
       ! U1 setvar "capture.channel1.port" "off"

```

647


SGD Printer Commands
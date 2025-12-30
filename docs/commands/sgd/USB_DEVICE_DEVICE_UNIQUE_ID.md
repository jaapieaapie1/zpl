# usb.device.device_unique_id



This command sets the USB Unique Device ID setting. The identifier that makes any printer unique is set
by the `"usb.device.serial_string"` command which is reported to the USB driver.

By default `"usb.device.serial_string"` reports the printer’s serial number. If
`"usb.device_unique_id"` is set to `"off"` the printer will report the `usb.device.serial_string`
parameter as its product family (e.g. ZT230, etc).

When subsequent printers of the same model, with `"usb.device_unique_id"` parameter to `"off"`, are
connected via USB, the host computer will not treat them as a new Plug and Play events, nor require new
driver installations.


**Setvar**


To set the current USB Unique Device Id setting:

```
       ! U1 setvar "usb.device.device_unique_id" "value"

```

**Values**

              - `"off"`

              - `"on"`

**Default**
```
          "on"

```

**Getvar**


To return the current USB Unique Device Id setting stored in the printer:

```
       ! U1 getvar "usb.device.device_unique_id"

```

**Example**

```
       ! U1 setvar "usb.device.device_unique_id" "off"

```

1012


SGD Printer Commands
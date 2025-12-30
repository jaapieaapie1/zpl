# bluetooth.minimum_security_mode



This printer setting parameter sets the Bluetooth Minimum Security mode. Minimum Security Mode
provides for three levels of security, depending on the printer radio version and printer firmware: “1”, “2”,
and “3”.


**IMPORTANT:** This feature is available in printers with Bluetooth® radio version 2.0 or higher.


**Setvar**


To set the Bluetooth Minimum Security mode:

```
       ! U1 setvar "bluetooth.minimum_security_mode" "value"

```

**Values**
`"1"` Security mode 1 means the printer will:

              - be discoverable


              - not require a PIN to connect to

`"2"` Security mode 2 means the printer will:

              - switch to bluetooth.authentication = “setpin” (Connecting device must provide the printer’s
bluetooth.bluetooth_pin),


              - switch to Bluetooth.encryption = “on”, and


              - use existing Bluetooth.discoverable setting

`"3"` Security mode 3 means Link Level Enforced Security and the printer will:

              - switch to bluetooth.authentication = “setpin” (Connecting device must provide the printer’s
bluetooth.bluetooth_pin)


              - switch to bluetooth.encryption = “on”


              - switch to bluetooth.discoverable = “off”


              - only connect to devices which use Link Level Enforced Security

`"4"` Security mode 4 means the printer will:

              - switch to bluetooth.authentication = “setpin” (Connecting device must provide the printer’s
bluetooth.bluetooth_pin)


              - switch to bluetooth.encryption = “on”


              - switch to bluetooth.discoverable = “off”


              - only connect to devices which use Link Level Enforced Security


              - will not connect to a device with Bluetooth version 2.1 or lower


**Default**
```
          "1"

```

1107


SGD Network Commands


**Default**


    - **For printers purchased in the EMEA region after August 1, 2025:**

`"2"` on printers with no front panel display

`"3"` on printers with a front panel display

    - **For all other printers:** `"1"`


**Getvar**


To retrieve the current Bluetooth Minimum Security Mode:

```
! U1 getvar "bluetooth.minimum_security_mode"

```

**Example**

This `setvar` example shows the value set to `"2"` .

```
! U1 setvar "bluetooth.minimum_security_mode" "2"

```

1108


SGD Network Commands
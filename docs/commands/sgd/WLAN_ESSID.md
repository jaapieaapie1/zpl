# wlan.essid



This printer setting refers to the printer’s stored ESSID.


**Setvar**


To instruct the printer to change the ESSID:

```
       ! U1 setvar "wlan.essid" "value"

```

**Values**


32 character alphanumeric string


**Default**

`"125"` (Printers running Link-OS v5.3 or earlier versions)

`""` (For Printers running Link-OS 6 or later versions)


**Getvar**


To instruct the printer to respond with the stored ESSID value:

```
       ! U1 getvar "wlan.essid"

```

**Example**

This `setvar` example shows the value set to `"125"` .

```
       ! U1 setvar "wlan.essid" "125"

```

When the `setvar` value is set to `"125"`, the `getvar` result is `"125"` .


**NOTE:** For Link-OS Firmware earlier than 6.0, setting ESSID to "" allows the printer to attempt to
connect to an AP with any ESSID. For Link-OS Firmware of 6.0 or later, setting ESSID to "" means
that the printer will not attempt a Wi-Fi connection.


1401


SGD Network Commands
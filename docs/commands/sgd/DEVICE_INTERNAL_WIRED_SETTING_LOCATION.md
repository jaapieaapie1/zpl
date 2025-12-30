# device.internal_wired_setting_location



This command identifies the location from where internal_wired network specific settings should be
retrieved.


**Setvar**


To specify the location from where internal_wired network specific settings should be retrieved:

```
       ! U1 setvar "device.internal_wired_setting_location" "value"

```

**Values**

              - `"network card"`

              - `"printer"`


**NOTE:** `"printer"` is the only valid option for the QLn series and ZD500 series printers.


**Default**
```
          "network_card"

```

**Getvar**


To display the location where internal_wired network specific settings are retrieved from:

```
       ! U1 getvar "device.internal_wired_setting_location"

```

**NOTE:** `"printer"` is the only valid getvar option for the QLn series and ZD500 series printers.


710




SGD Printer Commands
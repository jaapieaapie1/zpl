# wlan.enable



This parameter can be used to enable or disable the Wireless LAN functionality of the printer.


**Setvar**


To enable or disable printer’s Wireless LAN functionality:

```
       ! U1 setvar "wlan.enable" "value"

```

**Values**

              - `"on"` Wireless LAN functionality is enabled

              - `"off"` Wireless LAN functionality is disabled

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.enable"

```

1396


SGD Network Commands
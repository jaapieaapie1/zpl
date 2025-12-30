# device.restore_defaults



This command restores to the default of all settings within the specified SGD branch.


**Setvar**


To restore the default of all settings within the specified branch:

```
       ! U1 setvar "device.restore_defaults" "value"

```

**Values**

              - `"ip"` default all parameters in the IP branch

              - `"wlan"` default all parameters in the wlan branch

              - `"internal_wired"` default all parameters in the internal wired branch


**Do**


To restore the default of all settings within the specified branch:

```
       ! U1 do "device.restore_defaults" "value"

```

**Values**

              - `"ip"` default all parameters in the ip branch

              - `"wlan"` default all parameters in the wlan branch

              - `"internal_wired"` default all parameters in the internal wired branch


**Example**

These `do` and `setvar` examples restore the network card’s WLAN parameters to their default values.

```
       ! U1 do "device.restore_defaults" "wlan"
       ! U1 setvar "device.restore_defaults" "wlan"

```

745


SGD Printer Commands
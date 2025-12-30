# device.allow_firmware_downloads



This command sets if the firmware downloads are allowed.


**Setvar**


To set the command:

```
       ! U1 setvar "device.allow_firmware_downloads" "value"

```

**Values**

              - `"yes"` allow firmware downloads

              - `"no"` does not allow firmware downloads

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"no"`

              - **For all other printers:** `"yes"`


**Getvar**


To view the current setting:

```
       ! U1 getvar "device.allow_firmware_downloads"

```

**Example**

This `setvar` example sets the firmware downloads feature to `"no"` .

```
       ! U1 setvar "device.allow_firmware_downloads" "no"

```

663


SGD Printer Commands
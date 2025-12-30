# device.download_interactive_mode



This command enables an interactive firmware download. When enabled, the printer sends status
messages to the host as the firmware file is sent and processed. The status messages are in JSON format
and are sent back over the same channel to which the firmware is being sent.


**Getvar**


To determine whether interactive mode is enabled:

```
       ! U1 getvar "device.download_interactive_mode"

```

**Values**

              - `"on"` indicates that the interactive mode is enabled

              - `"off"` indicates that the interactive mode is disabled

**Default**
```
          "off"

```

**Setvar**


To enable the interactive mode for firmware download:

```
       ! U1 setvar "device.download_interactive_mode" "on"

```

**Values**

              - `"on"` indicates that the interactive mode is enabled

              - `"off"` indicates that the interactive mode is disabled


**Restore Default Setting**


To set the printer to the default setting for firmware download interactive mode:

```
       ! U1 setvar "device.restore_defaults" "device.download_interactive_mode"

```

685


SGD Printer Commands
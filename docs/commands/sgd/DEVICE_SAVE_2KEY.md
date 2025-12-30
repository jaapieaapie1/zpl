# device.save_2key



Sets or retrieves the current `device.save_2key` setting.


**NOTE:** The two-key report is a configuration listing used on legacy mobile printers.


**Setvar**

To set the current `device.save_2key` setting:

```
       ! U1 setvar "device.save_2key" "value"

```

**Values**

              - `"on"` Two-key diagnostics reports will be saved to Flash memory whenever a two-key report is
printed. The file will be named 2KEY.TXT.

              - `"off"` Two-key reports will not be saved to Flash memory.

              - `"now"` This choice can be used to generate a two-key diagnostics report on demand and save
it to Flash memory (save only, does not print). This choice does not alter the "on"/"off" state of
this SGD.


**Default**
```
          "on"

```

**Getvar**


To retrieve the current device.save_2key setting:

```
       ! U1 getvar "device.save_2key"

```

**Example**


This example instructs the printer to generate a two-key diagnostics report and save it to Flash memory.

```
       ! U1 setvar "device.save_2key" "now"

```

747


SGD Printer Commands
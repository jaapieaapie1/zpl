# file.capture_response.destination



Sets where a log file is saved that logs data coming from the language parsers to a host (where data was
originally sent).


**Setvar**

Use `file.capture_response.begin` and `file.capture_response.end` to start and stop logging.

To set where a log file is saved:

          - `"printer_file"` causes captured files to be written to the printer’s memory

          - `"usb_file"` causes captured files to be written to a USB storage device

**Default**
```
          "printer_file"

```

**Example**

```
       ! U1 setvar "file.capture_response.destination" "usb_file"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "file.capture_response.destination"

```

**Result**

```
          "usb_file"

```

819


SGD Printer Commands
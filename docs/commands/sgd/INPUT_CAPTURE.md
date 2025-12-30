# input.capture



This parameter allows capturing input data in diagnostics mode. Input capture has three modes:,
`"print"`, `"usb_host"`, `"run"`, and `"off"` . The `"print"`, `"usb_host"`, and `"run"` modes can be used
to examine data received by the printer.

When in `"print"` mode the printer will save incoming data to files named `"in???.dmp",` where `???` is
a number between 001 to 999. The printer will then print the text and hexadecimal representation of data
bytes received instead of printing the formatted labels which that data might represent.

When in `"usb_host"` mode the printer will save incoming data to files named named `"in???.dmp",`
where `???` is a number between 001 to 999. The printer will then save the files to a USB Host memory
device if present.

When in `"run"` mode the printer will save captured incoming data to files as above, but will otherwise run
the incoming data/commands normally.


The capture files should be deleted from printer memory after retrieving them. Leaving the printer in
`"print"` or `"run"` mode and not deleting the capture files will reduce the printer’s available flash
memory.

The `"off"` mode is the printer’s normal operating mode. Cycling power will also return the printer to
`"off"` mode.


**NOTE:** This command does not capture a network packet trace.


**Setvar**


To set the directory name from which to retrieve files:

```
       ! U1 setvar "input.capture" "value"

```

**Values**

              - `"print"`

              - `"usb_host"`

              - `"run"`

              - `"off"`


**Getvar**

To retrieve the current `input.capture` setting value:

```
       ! U1 getvar "input.capture"

```

**NOTE:**

The `device.diagnostic_print` setting is deprecated.

Setting `device.diagnostic_print` to `"enabled"` is the equivalent of setting
`input_capture` to `"print"` .

```
       ! U1 setvar "device.diagnostic_print" "enabled"

```

835


SGD Printer Commands


Setting `device.diagnostic_print` to `"disabled"` is the equivalent of setting
`input_capture` to `"off"` .

```
! U1 setvar "device.diagnostic_print" "disabled"

```

836


SGD Printer Commands
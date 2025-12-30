# usb.host.write_list



This command scans the `E:` flash drive for non-user-restricted files and displays the files in a list.


**Setvar**

To scan the `E:` flash drive for non-user-restricted files and displays the files in a list:

```
       ! U1 setvar "usb.host.write_list" "value"

```

**Values**

              - `"fill_store"` The printer analyzes the contents of its `E:` drive and creates a list of files that
may be copied to a USB mass storage device connected to the printer.

              - `"store"` The printer copies the current file (the file listed on the display) or all eligible files if
`"SELECT ALL"` is displayed to the USB mass storage device.

              - `"up"`

              - `"down"`

**Default**
```
          "on"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "usb.host.write_list"

```

**Result**


One of the following:


The current file in the list.

              - `"NONE"` if there are no files on the `E:` drive of the printer.

              - `"READONLY"` if the USB mass storage device is read-only.


1031


SGD Printer Commands
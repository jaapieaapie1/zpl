# usb.host.read_list



This command scans the mounted usb mass storage device for non-user-restricted files and displays the
files in a list.


**Setvar**


To scan the mounted usb mass storage device for non-user-restricted files and displays the files in a list:

```
       ! U1 setvar "usb.host.read_list" "value"

```

**Values**

              - `"fill_store"` The printer creates a list of all of the top-level files that are readable on a
connected mass storage device. ( `.ZPL` and `.XML` files)

              - `"store"` The printer copies the current file (the file listed on the display) or all eligible files if
`SELECT ALL` is displayed from the USB mass storage device to the printer’s `E:` drive.


**NOTE:** `"store"` does an exact byte-for-byte copy of the file being stored. ZPL files
that are transferred to the printer using the “store” command are NOT prepared to
be used with a recall format command (^XF). See XREF Character Substitution (in
Mirror) for the modifications necessary to prepare a ZPL file for use with ^XF. If the
ZPL file contains a ^DF, the appropriate way to place it on the printer’s memory is to
use the `"print"` command, below.

              - `"fill_print"` The printer analyzes the content of the USB mass storage device and creates a
list of files that may be printed. ( `.ZPL` files only)

              - `"print"` The printer prints the current file (the file listed on the display) or all eligible files if
`SELECT ALL` is displayed

              - `"clear"`


**Getvar**


To return the current setting value:

```
       ! U1 getvar "usbusb.host.read_list"

```

**Result**

The current file in the list or `"NONE"` .


**Example**

```
       ! U1 getvar "usbusb.host.read_list"

```

One of the following:


          - The current file in the list.

          - `"NONE"` if there are no files on the USB mass storage device.


1027


SGD Printer Commands
# usb.host.config_info_to_usb



This command is used by WML to save the ^HH output files to the USB thumb drive. The file names are
written in the CONFIGxxx.TXT format, where xxx ranges from 1 to 999.


**Setvar**

To enable or disable saving the `^HH` output to the USB thumb drive:

```
       ! U1 setvar "usb.host.config_info_to_usb" "values"

```

**Values**

`"yes"` means `^HH` output is saved

`"no"` means no action is taken


**Example**

In the setvar example below, the `^HH` output file is saved to the USB thumb drive.

```
       ! U1 setvar "usb.host.config_info_to_usb" "yes"

```

1020




SGD Printer Commands
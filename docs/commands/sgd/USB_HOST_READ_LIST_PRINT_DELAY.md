# usb.host.read_list_print_delay



This command specifies a number of milliseconds for the printer to wait before processing the next file
when `“SELECT ALL”` is chosen on the `USB PRINT FILES` user menu.


**Setvar**


To specify the print delay time in milliseconds:

```
       ! U1 setvar "usb.host.read_list_print_delay"

```

**Values**

`"0"` to `"65535"`

**Default**
```
          "0"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "usb.host.read_list_print_delay"

```

1028


SGD Printer Commands
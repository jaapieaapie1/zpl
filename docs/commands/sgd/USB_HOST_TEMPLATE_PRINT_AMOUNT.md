# usb.host.template_print_amount



This command is used by WML to control the number of template-type labels to print.


**Setvar**


To control the number of template-type labels to print:

```
       ! U1 setvar "usb.host.template_print_amount" "value"

```

**Values**

              - `"1"` to `"99999"`

              - `"up"` moves to the previous file in the list

              - `"down"` moves to the next file in the list

**Default**

`"1"` which will change based on the contents of the selected template file


**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "usb.host.template_print_amount"

```

**Result**


The current value.


1030




SGD Printer Commands
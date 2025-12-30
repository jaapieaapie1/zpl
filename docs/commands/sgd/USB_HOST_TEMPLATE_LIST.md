# usb.host.template_list



This command is used by WML to support selecting `.ZPL` template files resident on the `E:/` drive.


**Setvar**

To list the `.ZPL` files from the E drive:

```
       ! U1 getvar "usb.host.template_list"

```

**Values**

              - `"fill"` The printer analyzes the contents of its `E:/` drive and creates a list of the template files
with a `.ZPL` extension.

              - `"up"` moves to the previous file in the list

              - `"down"` moves to the next file in the list

**Default**
```
          "on"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "usb.host.template_list"

```

**Result**


One of the following:

              - `"UNINITIALIZED"`

              - `"IN PROGRESS"`

              - `"NONE"` if no `.ZPL` files reside on the `E:/` drive

              - the current `.ZPL` file


1029


SGD Printer Commands
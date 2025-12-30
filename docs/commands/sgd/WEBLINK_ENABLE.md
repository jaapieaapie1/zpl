# weblink.enable



This command indicates if one or more of the weblink connections are active.

If there is more than one connection under the weblink branch (for example, `weblink.ip.conn1` ) and if
any of the `.location` values are set, then this SGD will be set to `"on"` . If all connections are disabled (all
connection `.location` values set to `""` ), then this value will be set to `"off"` .

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Getvar**


To indicate if one or more of the weblink connections are active:

```
       ! U1 getvar "weblink.enable"

```

**Result**

              - `"yes"` if any of the location values are set

              - `"off"` if all connections are disabled


1331


SGD Network Commands
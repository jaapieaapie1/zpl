# ip.tcp.enable



This printer setting refers to the TCP socket protocol.


**Setvar**


To turn the TCP on or off:

```
       ! U1 setvar "ip.tcp.enable" "value"

```

**Values**

`"off"` disables TCP protocol

`"on"` enables TCP protocol

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To respond with the TCP status:

```
       ! U1 getvar "ip.tcp.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.tcp.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1324


SGD Network Commands
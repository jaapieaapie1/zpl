# ip.udp.enable



This printer setting refers to the UDP socket protocol.


**Setvar**


To turn UDP on or off:

```
       ! U1 setvar "ip.udp.enable" "value"

```

**Values**

`"off"` disables UDP protocol

`"on"` enables UDP protocol

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To respond with the UDP status:

```
       ! U1 getvar "ip.udp.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.udp.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1329


SGD Network Commands
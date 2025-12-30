# ip.lpd.enable



This printer setting refers to the LPD (Line Printer Daemon) protocol setting.


**IMPORTANT:** LPD communications from the host should be directed to port 515.


**Setvar**


To turn LPD on or off:

```
       ! U1 setvar "ip.lpd.enable" "value"

```

**Values**

`"off"` disables LPD protocol

`"on"` enables LPD protocol

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To respond with the LPD status:

```
       ! U1 getvar "ip.lpd.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.lpd.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1266


SGD Network Commands
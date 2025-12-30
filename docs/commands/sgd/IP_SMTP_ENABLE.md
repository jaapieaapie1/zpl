# ip.smtp.enable



This printer setting refers to the SMTP protocol.


**Setvar**


To turn SMTP on or off:

```
       ! U1 setvar "ip.smtp.enable" "value"

```

**Values**

`"off"` disables SMTP

`"on"` enables SMTP

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To return the SMTP status:

```
       ! U1 getvar "ip.smtp.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.smtp.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on".`


1311


SGD Network Commands
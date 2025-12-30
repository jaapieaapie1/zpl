# ip.pop3.enable



This printer setting determines if the printer queries a POP3 mailbox for mail.


**Setvar**


To turn POP3 on or off:

```
       ! U1 setvar "ip.pop3.enable" "value"

```

**Values**

`"off"` disables POP3

`"on"` enables POP3

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To respond with the POP3 status:

```
       ! U1 getvar "ip.pop3.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "ip.pop3.enable" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1295


SGD Network Commands
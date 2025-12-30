# ip.pop3.poll



This printer setting refers to how frequent (in seconds) the printer queries a POP3 mailbox for new mail.
This only applies if the " `ip.pop3.enable` " is set to on.


**Setvar**


To change the POP3 poll interval:

```
       ! U1 setvar "ip.pop3.poll" "value"

```

A value of `"0"` causes the printer to only query the POP3 mailbox one time, on printer power up, or
following a network reset.


**Values**

`"0"` through `"65535"`

**Default**

```
          "0"

```

**NOTE:** A poll value of less then thirty seconds is not recommended. The printer is
unresponsive for several seconds when polling for email depending on data transfer
time from the server to the printer.


**Getvar**


To respond with the POP3 poll frequency (in seconds):

```
       ! U1 getvar "ip.pop3.poll"

```

**Example**

This `setvar` example shows the value set to `"0"` .

```
       ! U1 setvar "ip.pop3.poll" "0"

```

When the `setvar` value is set to `"0"`, the `getvar` result is `"0"` .


1297


SGD Network Commands
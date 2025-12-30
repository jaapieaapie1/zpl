# ip.pop3.password



This printer setting refers to the POP3 mailbox password. This only applies if " `ip.pop3.enable` " is set to
on.


**Setvar**


To change the POP3 password:

```
       ! U1 setvar "ip.pop3.password" "value"

```

**Values**


A maximum of 20 alphanumeric characters


**Default**
```
          " "

```

**Getvar**


To respond with the POP3 password:

```
       ! U1 getvar "ip.pop3.password"

```

For protection a single `"*"` prints.


**Example**

This `setvar` example shows the value set to `"password"` .

```
       ! U1 setvar "ip.pop3.password" "password"

```

When the `setvar` value is set to `"password"`, the `getvar` result is `"*"` .


1296


SGD Network Commands
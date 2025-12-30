# ip.pop3.username



This printer setting refers to the POP3 user name. This only applies if the " `ip.pop3.enable` " is set to on.


**Setvar**


To change the POP3 user name:

```
       ! U1 setvar "ip.pop3.username" "value"

```

**Values**


A maximum of 20 alphanumeric characters


**Default**
```
          ""

```

**Getvar**


To respond with the POP3 user name:

```
       ! U1 getvar "ip.pop3.username"

```

**Example**

This `setvar` example shows the value set to `"user"` .

```
       ! U1 setvar "ip.pop3.username" "user"

```

When the `setvar` value is set to `"user"`, the `getvar` result is `"user"` .


1302


SGD Network Commands
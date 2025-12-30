# ip.pop3.server_addr



This printer setting refers to the POP3 server IP address that the printer contacts when checking for new
mail. This only applies if " `ip.pop3.enable` " is set to on.


**Setvar**


To change the POP3 server address:

```
       ! U1 setvar "ip.pop3.server_addr" "value"

```

**Values**


Any valid POP3 server address


**Default**
```
          "0.0.0.0"

```

**Getvar**


To respond with the POP3 server address:

```
       ! U1 getvar "ip.pop3.server_addr"

```

1301


SGD Network Commands
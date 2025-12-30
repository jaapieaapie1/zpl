# ip.smtp.domain



This printer setting refers to the domain name used by the printer in sending email with respect to the
SMTP server.


**Setvar**


To change the SMTP domain name:

```
       ! U1 setvar "ip.smtp.domain" "value"

```

**Values**


A maximum of 24 alphanumeric characters.


**Default**
```
          "ZBRPrintServer"

```

**Getvar**


To return the SMTP domain name:

```
       ! U1 getvar "ip.smtp.domain"

```

**Example**

This `setvar` example shows the value set to `"ZBRPrintServer.com"` .

```
       ! U1 setvar "ip.smtp.domain" "ZBRPrintServer.com"

```

When the `setvar` value is set to `"ZBRPrintServer.com"`, the `getvar` result is
`"ZBRPrintServer.com"` .


1310




SGD Network Commands
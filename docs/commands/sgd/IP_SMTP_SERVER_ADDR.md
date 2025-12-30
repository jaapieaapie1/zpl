# ip.smtp.server_addr



This printer setting refers to the IP address of the SMTP server used for sending email.


**Setvar**


To change the SMTP server address:

```
       ! U1 setvar "ip.smtp.server_addr" "value"

```

**Values**


Any valid IP address.


**Default**
```
          0.0.0.0

```

**Getvar**


To respond with the current SMTP server address:

```
       ! U1 getvar "ip.smtp.server_addr"

```

**Example**

This `setvar` example shows the value set to `10.10.10.10` .

```
       ! U1 setvar "ip.smtp.server_addr" "10.10.10.10"

```

When the `setvar` value is set to `"10.10.10.10"`, the `getvar` result is `"10.10.10.10"` .


1312


SGD Network Commands
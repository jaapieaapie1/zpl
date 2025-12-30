# ip.ftp.request_password



This command controls whether the ftp client will prompt the user to enter a password at the beginning of
an ftp session.


**Setvar**


To turn on or off the ftp session password request:

```
       ! U1 setvar "ip.ftp.request_password" "value"

```

**Values**
```
          "no"
          "yes"

```

**Default**
```
       "no"

```

**Getvar**


To return whether the ftp client is set to prompt the user for a password at the start of an ftp session:

```
       ! U1 getvar "ip.ftp.request_password"

```

1253


SGD Network Commands
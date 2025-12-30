# wlan.kerberos.username



This printer setting refers to the Kerberos user name. The user name must correspond to a user profile
established on the Kerberos KDC server in use.


**Setvar**


To change the Kerberos user name:

```
       ! U1 setvar "wlan.kerberos.username" "value"

```

**Values**

`"0"` to `"32"` alphanumeric characters

**Default**
```
          "user"

```

**Getvar**


To respond with the current Kerberos user name:

```
       ! U1 getvar "wlan.kerberos.username"

```

**Example**

This `setvar` example shows the value set to `"user"` .

```
       ! U1 setvar "wlan.kerberos.username" "user"

```

When the `setvar` value is set to `"user"`, the `getvar` result is `"user"` .


1455


SGD Network Commands
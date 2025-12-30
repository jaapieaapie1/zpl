# wlan.kerberos.password



This printer setting refers to the Kerberos password. The password must correspond to a user profile
established on the Kerberos KDC server in use.


**Setvar**


To set the Kerberos password:

```
       ! U1 setvar "wlan.kerberos.password" "value"

```

**Values**

`"0"` through `"32"` alphanumeric characters

**Default**
```
          "password"

```

**Getvar**


To respond with the current Kerberos password:
```
       ! U1 getvar "wlan.kerberos.password"
```

For protection a single `"*"` prints.


**Example**

This `setvar` example shows the value set to `"password"` .

```
       ! U1 setvar "wlan.kerberos.password" "password"

```

When the `setvar` value is set to `"password"`, the `getvar` result is `"*"` .


1453


SGD Network Commands
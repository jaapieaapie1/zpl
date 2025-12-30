# wlan.password



This printer setting refers to the generic password that is used by the wireless securities that need a
password.


**IMPORTANT:** Kerberos has its own password field.


**Setvar**


To set a generic password for the wireless securities that need a password:

```
       ! U1 setvar "wlan.password" "value"

```

**Values**


A maximum of 32 alphanumeric characters.


**Default**
```
          "password"

```

**Getvar**


To respond with a generic password for wireless securities:

```
       ! U1 getvar "wlan.password"

```

For protection a single `"*"` prints.


**Example**

This `setvar` example shows the value set to `"password"` .

```
       ! U1 setvar "wlan.password" "password"

```

When the `setvar` value is set to `"password"`, the `getvar` result is `"*"` .


1462


SGD Network Commands
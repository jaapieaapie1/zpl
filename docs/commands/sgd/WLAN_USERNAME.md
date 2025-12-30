# wlan.username



This printer setting refers to the generic user name that is used by the wireless securities that need a user
name.


**IMPORTANT:** Kerberos has its own user name field.


**Setvar**


To set a generic user name for wireless securities that need a user name:

```
       ! U1 setvar "wlan.username" "value"

```

**Values**


A maximum of 32 alphanumeric characters


**Default**
```
          "user"

```

**Getvar**


To respond with a generic user name for the wireless securities that need a user name:

```
       ! U1 getvar "wlan.username"

```

**Example**

This `setvar` example shows the value set to `"user"` .

```
       ! U1 setvar "wlan.username" "user"

```

When the `setvar` value is set to `"user"`, the `getvar` result is `"user"` .


1495


SGD Network Commands
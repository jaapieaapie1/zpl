# wlan.private_key_password



This printer setting allows the setting of the optional private key password.


**Setvar**


This command instructs the printer to set the private key password.

```
       ! U1 setvar "wlan.private_key_password" "value"

```

**Values**
A maximum of 32 alphanumeric characters

**Default**
```
          ""

```

**Getvar**


This command instructs the printer to respond with the value of the private key password.

```
       ! U1 getvar "wlan.private_key_password"

```

For protection a single `"*"` prints.


**Example**

This `setvar` example shows the value set to `"password"` .

```
       ! U1 setvar "wlan.private_key_password" "password"

```

When the `setvar` value is set to `"password"`, the `getvar` result is `"*"` .


1467


SGD Network Commands
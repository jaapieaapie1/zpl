# ip.mirror.password



This command specifies the user password on the mirroring server assigned for mirroring updates (fetch).


**Setvar**


To specify a password for mirroring updates (fetch):

```
       ! U1 setvar "ip.mirror.password" "value"

```

**Values**


Alphanumeric text string (1 to 20 characters)


**Default**
```
          "password"

```

**Getvar**


To retrieve the user password the printer is currently using for mirroring updates (fetch):

```
       ! U1 getvar "ip.mirror.password"

```

**Example**

This `setvar` example shows the value set to `"password"` .

```
       ! U1 setvar "ip.mirror.password" "password"

```

When the `setvar` value is set to `"password"`, the `getvar` result is `"*"` . For security purposes, the
printer does not return password information.


1282


SGD Network Commands
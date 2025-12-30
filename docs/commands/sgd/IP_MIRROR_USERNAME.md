# ip.mirror.username



This command specifies the user name on the mirroring server assigned for mirroring updates (fetch).


**Setvar**


To set a specific user name for mirroring updates (fetch):

```
       ! U1 setvar "ip.mirror.username" "value"

```

**Values**


Alphanumeric text string (1 to 20 characters)


**Default**
```
          "user"

```

**Getvar**


To retrieve the user name the printer is currently using for mirroring updates (fetch):

```
       ! U1 getvar "ip.mirror.username"

```

**Example**

This `setvar` example shows the value set to `"user"` .

```
       ! U1 setvar "ip.mirror.username" "user"

```

When the `setvar` value is set to `"user"`, the `getvar` result is `"user"` .


1288


SGD Network Commands
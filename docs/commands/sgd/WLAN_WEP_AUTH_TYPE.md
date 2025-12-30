# wlan.wep.auth_type



For the WEP security type, this printer setting selects the authentication type to be used between the
printer and the access point. The authentication types are open system and shared key.


**NOTE:** This command is not supported for printers running Link-OS 6 or later versions.


**Setvar**


To set the WEP authentication type:

```
       ! U1 setvar "wlan.wep.auth_type" "value"

```

**Values**

`"open"` enables the open authentication type

`"shared"` enables the shared authentication type


**Getvar**


To retrieve the current WEP authentication type:

```
       ! U1 getvar "wlan.wep.auth_type"

```

**Example**

This `setvar` example shows the value set to `"open"` .

```
       ! U1 setvar "wlan.wep.auth_type" "open"

```

When the `setvar` value is set to `"open"`, the `getvar` result is `"open"` .


1496


SGD Network Commands
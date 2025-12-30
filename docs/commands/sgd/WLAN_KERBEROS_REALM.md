# wlan.kerberos.realm



This printer setting refers to the Kerberos realm, an administrative domain with its own Kerberos server
(KDC).


**IMPORTANT:** If you are using a Windows 2000 Server the realm must be all upper-case. For
details, see example below.


**Setvar**


To change the Kerberos realm:

```
       ! U1 setvar "wlan.kerberos.realm" "value"

```

**Values**

`"0"` through `"64"` alphanumeric characters

**Default**
```
          "kerberos"

```

**Getvar**


To respond with the current Kerberos realm:

```
       ! U1 getvar "wlan.kerberos.realm"

```

**Example**

This `setvar` example shows the value set to `"zebra"` .

```
       ! U1 setvar "wlan.kerberos.realm" "zebra"

```

When the `setvar` value is set to `"zebra"`, the `getvar` result is `"zebra"` .

This `setvar` example shows the value set to `"ZEBRA"` on a Windows 2000 server.

```
       ! U1 setvar "wlan.kerberos.realm" "ZEBRA"

```

When the `setvar` value is set to `"ZEBRA"`, the `getvar` result is `"ZEBRA"` .


1454


SGD Network Commands
# wlan.kerberos.mode



This printer setting refers to the Kerberos network authentication protocol. Kerberos provides secure
mutual authentication for a wireless client through a Symbol Access Point, based on user information
stored on a Kerberos KDC (Key Distribution Center) server.


**Setvar**


To enable or disable the Kerberos mode:

```
       ! U1 setvar "wlan.kerberos.mode" "values"

```

**Values**

`"off"` disables Kerberos mode

`"on"` enables Kerberos mode

**Default**
```
          "off"

```

**Getvar**


To respond with the current Kerberos mode:

```
       ! U1 getvar "wlan.kerberos.mode"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "wlan.kerberos.mode" "on"

```

When the `setvar` value is set to `"on"`, the `getvar` result is `"on"` .


1452


SGD Network Commands
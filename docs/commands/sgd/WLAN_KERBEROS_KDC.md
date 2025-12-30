# wlan.kerberos.kdc



This printer setting refers to the Kerberos Key Distribution Center (KDC). The KDC is a trusted server which
maintains a database with account information for all security principals (users) for a particular site or
administrative domain (realm).


**Setvar**


To change the Kerberos KDC:

```
       ! U1 setvar "wlan.kerberos.kdc" "value"

```

**Values**

`"0"` to `"32"` ASCII characters

**Default**
```
          "krbtgt"

```

**Getvar**


To respond with the current Kerberos KDC:

```
       ! U1 getvar "wlan.kerberos.kdc"

```

**Example**

This `setvar` example shows the value set to `"krbtgt"` .

```
       ! U1 setvar "wlan.kerberos.kdc" "krbtgt"

```

When the `setvar` value is set to `"krbtgt"`, the `getvar` result is `"krbtgt"`


1451


SGD Network Commands
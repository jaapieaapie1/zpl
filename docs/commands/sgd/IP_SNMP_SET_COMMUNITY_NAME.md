# ip.snmp.set_community_name



This printer setting is used when changing SNMP data remotely. To alter any SNMP data, the SNMP client
must supply the set community name that matches the printer’s set community name.


**Setvar**


To set the SNMP set community name string:

```
       ! U1 setvar "ip.snmp.set_community_name" "value"

```

**Values**


A maximum of 19 alphanumeric characters


**Default**
```
          "public"

```

**Getvar**


To return the printer’s SNMP set community name string:

```
       ! U1 getvar "ip.snmp.set_community_name"

```

For protection a single `"*"` returns.


**Example**

This `setvar` example shows the value set to `"public"` .

```
       ! U1 setvar "ip.snmp.set_community_name" "public"

```

When the `setvar` value is set to `"public"`, the `getvar` result is `"*"` .


1315


SGD Network Commands
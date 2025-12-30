# ip.snmp.get_community_name



This printer setting is used when making SNMP queries. The SNMP client must supply the get community
name that matches the printer’s get community name in order to query any SNMP data.


**Setvar**


To set the SNMP get community name string:

```
       ! U1 setvar "ip.snmp.get_community_name" "value"

```

**Values**


A maximum of 19 alphanumeric characters.


**Default**
```
          "public"

```

**Getvar**


To get the SNMP get community name string:

```
       ! U1 getvar "ip.snmp.get_community_name"

```

For protection a single "*" prints.


**Example**

This `setvar` example shows the value set to `"public"` .

```
       ! U1 setvar "ip.snmp.get_community_name" "public"

```

When the `setvar` value is set to `"public"`, the `getvar` result is `"*"` .


1314


SGD Network Commands
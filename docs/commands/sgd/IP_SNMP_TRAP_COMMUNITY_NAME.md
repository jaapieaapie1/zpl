# ip.snmp.trap_community_name



This command sets the SNMP Trap Community name of the print server.


**Setvar**


To set the SNMP trap community name:

```
       ! U1 setvar "ip.snmp.get_community_name" "value"

```

**Values**


A maximum of 20 alphanumeric characters.


**Default**
```
          "public"

```

**Getvar**


To get the SNMP trap community name:

```
       ! U1 getvar "ip.snmp.trap_community_name"

```

**Example**

```
       ! U1 setvar "ip.snmp.trap_community_name" "public"

```

1316


SGD Network Commands
# mqtt.conn[1|2].tenant_id



This command specifies the top level of the MQTT topic the printer will subscribe to and publish to on
connection 1 or 2.


**Setvar**


To set the top-level topic that the printer subscribes and publishes to on connection 1 or 2:

```
       ! U1 setvar "mqtt.conn1.tenant_id" "value"

       ! U1 setvar "mqtt.conn2.tenant_id" "value"

```

**Values**
1 to 64 non-whitespace ASCII characters and must not include a `+`, `#`, `/`, or `$` .

**Default**
```
          "zebra"

```

**Getvar**


To have the printer return the current setting value for connection 1 or 2:

```
       ! U1 getvar "mqtt.conn1.tenant_id"

       ! U1 getvar "mqtt.conn2.tenant_id"

```

**Result**
The printer returns the tenant ID.


897


SGD Printer Commands
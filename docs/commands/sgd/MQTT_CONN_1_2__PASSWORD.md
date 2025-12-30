# mqtt.conn[1|2].password



This command specifies the password to use for MQTT connection 1 or 2. If set to `""`, a password is not
sent to the broker. MQTT 3.1.1 brokers require either username only, username and password, or neither. If
only a password is set, a warning is issued in the syslog.


**Setvar**


To set the password for MQTT connection 1 or 2:

```
       ! U1 setvar "mqtt.conn1.password" "value"

       ! U1 setvar "mqtt.conn2.password" "value"

```

**Value**
A string with a maximum of 64 characters. If the password is empty, it is not sent to the broker.

**Default**
```
          ""

```

**Getvar**


To have the printer return the password for connection 1 or 2:

```
       ! U1 getvar "mqtt.conn1.password"

       ! U1 getvar "mqtt.conn2.password"

```

**Result**
The printer returns `*` and does not return the password


891


SGD Printer Commands
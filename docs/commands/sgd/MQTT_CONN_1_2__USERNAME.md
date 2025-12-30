# mqtt.conn[1|2].username



This command specifies the username to use for MQTT connection 1 or 2. If the username is set to `""`,
the default, the username is not sent to the broker. MQTT 3.1.1 brokers require either username only,
username and password, or neither. If only a password is sent, a warning is issued in syslog.


**Setvar**


To set the username for connection 1 or 2:

```
       ! U1 setvar "mqtt.conn1.username" "value"

       ! U1 setvar "mqtt.conn2.username" "value"

```

**Value**
String with a maximum of 64 characters.

**Default**
```
          ""

```

**Getvar**


To have the printer return the username for connection 1 or 2:

```
       ! U1 getvar "mqtt.conn1.username"

       ! U1 getvar "mqtt.conn2.username"

```

898


SGD Printer Commands
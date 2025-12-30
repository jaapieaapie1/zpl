# mqtt.conn[1|2].clean_session_flag



This command specifies whether to clear all previous values on MQTT connection 1 or MQTT connection 2.
When set to `"on"`, the default, the broker does not create a persistent session for new connections. If the
client disconnects, any messages in the queue are lost. When set to `"off"`, the broker creates a persistent
session and any messages in the queue persist if the client disconnects. The persisted messages are sent
when the client reconnects.


**Setvar**


To persist MQTT messages on MQTT connections:

```
       ! U1 setvar "mqtt.conn1.clean_session_flag" "value"

       ! U1 setvar "mqtt.conn2.clean_session_flag" "value"

```

**Values**

              - `"on"` (default) the broker does not persist MQTT message on connection 1.

              - `"off"` causes the broker to persist messages

**Default**
```
          "on"

```

**Getvar**


To have the printer return the value of the MQTT session flag for connection 1:

```
       ! U1 getvar "mqtt.conn1.clean_session_flag"

       ! U1 getvar "mqtt.conn2.clean_session_flag"

```

**Values**

              - `"on"` (default) the broker does not persist MQTT message on connection 1.

              - `"off"` causes the broker to persist messages


890




SGD Printer Commands
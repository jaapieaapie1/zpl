# mqtt.conn[1|2].ping_interval



This command specifies how often connection 1 or 2 sends a ping to the MQTT broker to keep the
connection open. This value is in seconds. If the MQTT broker does not receive a ping request within this
interval, the broker will disconnect from the client.


**Setvar**


To set the ping interval for connection 1 or 2:

```
       ! U1 setvar "mqtt.conn1.ping_interval" "value"

       ! U1 setvar "mqtt.conn2.ping_interval" "value"

```

**Value**
A numeric value between `"1"` and `"300"` seconds.

**Default**
`"30"` seconds


**Getvar**


To have the printer return the current ping interval for connection 1 or 2:

```
       ! U1 getvar "mqtt.conn1.ping_interval"

       ! U1 getvar "mqtt.conn2.ping_interval"

```

The printer returns a numeric value between `"1"` and `"300"` seconds.


892


SGD Printer Commands
# mqtt.conn[1|2].retry_interval_random_max



This command specifies the maximum random retry interval to attempt a connection to an MQTT broker
when the connection is lost. This command prevents a fleet of printers from attempting to connect to a
single broker after a network interruption that might cause denial of service (DOS) concerns. The interval is
measured in seconds.


**Setvar**


To set the maximum random retry interval for connection 1 or 2:

```
       ! U1 setvar "mqtt.conn1.retry_intreval_random_max" "value"

       ! U1 setvar "mqtt.conn2.retry_intreval_random_max" "value"

```

**Value**
A numeric value between `"1"` and `"600"` seconds.

**Default**
```
          "120"

```

**Getvar**


To have the printer return the current maximum random retry interval for connection 1 or 2:

```
       ! U1 getvar "mqtt.conn1.retry_intreval_random_max"

       ! U1 getvar "mqtt.conn2.retry_intreval_random_max"

```

The printer returns a numeric value between `"1"` and `"600"` seconds.


895


SGD Printer Commands
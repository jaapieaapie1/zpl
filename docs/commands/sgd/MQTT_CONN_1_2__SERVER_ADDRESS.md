# mqtt.conn[1|2].server_address



This command specifies the URI the printer client uses to connect to the MQTT broker for connection 1
or 2. The printer will not attempt to connect to the same server address on multiple connections or if the
value is invalid. This address must be a TLS MQTT broker (mqtts) as non-TLS broker connections are not
supported.


**Setvar**


To specify the URI for connection 1 or 2 to the MQTT broker:

```
       U1 setvar "mqtt.conn1.server_address" "mqtts://<domain>[:port][/path]"

       U1 setvar "mqtt.conn2.server_address" "mqtts://<domain>[:port][/path]"

```

**Values**

              - `<domain>` is a required value. DNS or IP address is an accepted value.

              - `[:port]` is an optional value. If the port is not specified, the default port of 8883 is used. A
value of zero (0) is not valid.

              - `[/path]` is an optional value.

The maximum length of the string is 2048 characters.


**Getvar**


To have the printer return the URI value for connection 1 or 2:

```
       ! U1 getvar "mqtt.conn1.server_address"

       ! U1 getvar "mqtt.conn2.server_address"

```

The command retrieves the URI.


**Example**

This example includes a DNS domain of `broker.zebra.com` using port 65412.

```
       U1 setvar "mqtt.conn1.server_address" "mqtts://broker.zebra.com:65412"

```

896


SGD Printer Commands
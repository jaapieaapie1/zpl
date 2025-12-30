# mqtt.conn[1|2].reset_required



This command shows if a reset is required in order for an MQTT setting to take effect. Any time a setting
for MQTT is changed, a connection reset is required in order for the change to take effect.


**Getvar**


To have the printer return whether a reset of connection 1 or 2 is required:

```
       ! U1 getvar "mqtt.conn1.reset_required"

       ! U1 getvar "mqtt.conn2.reset_required"

```

**Values**

              - `"no"` indicates a connection reset is not required.

              - `"yes"` indicates a connection reset is required.


894


SGD Printer Commands
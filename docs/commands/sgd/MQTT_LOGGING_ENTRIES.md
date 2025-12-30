# mqtt.logging.entries



This command returns an MQTT list of log entries for both mqtt1 (conn1) and mqtt2 (conn2) that are
created on printer power up and cleared after a power cycle or an explicit clearing. The number of entries
is determined by `mqtt.logging.max_entries` .


**Getvar**


To return a list of MQTT log entries for connection 1 and connection 2 from the printer:

```
       ! U1 getvar "mqtt.logging.entries"

```

**Value**
An ASCII string of log data.


**Example**


This example shows an example of a log entry.

```
       "[07-12-2021 21:31:29.733][Info][0000100F][mqtt1] Waiting 2 seconds "

```

887


SGD Printer Commands
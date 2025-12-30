# mqtt.logging.max_entries



This command sets the maximum number of log entries recorded before the least-recently-logged entry
is removed. A value of 0 disables recording MQTT logs to the `mqtt.logging.entries` SGD; log
entries are still recorded to syslog. Changing this number to a lower value than the number of log entries
currently recorded causes older entries to be removed until the total number of entries does not exceed
the maximum number of entries.


**Setvar**


To set the maximum number of MQTT log entries:

```
       ! U1 setvar "mqtt.logging.max_entries" "value"

```

**Value**
A numeric value between `"1"` and `"10,000"` . A value of zero ( `"0"` ) disables logging to
`mqtt.logging.entries` .

**Default**
```
          ""

```

**Getvar**

To return the setting for `mqtt.logging.max_entries` :

```
       ! U1 getvar "mqtt.logging.max_entries"

```

This command returns the current value for the maximum number of log entries listed in
`mqtt.logging.entries` .


888


SGD Printer Commands
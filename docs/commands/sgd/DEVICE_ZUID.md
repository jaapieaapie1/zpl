# device.zuid



Reports a unique ID for the printer that is used for the MQTT topic and MQTT client ID. The zuid is created
by the printer the first time it is needed and remains the same unless the printer is decommissioned, at
which point a new unique ID is created.


**Getvar**


To retrieve the unique ID for the printer:

```
       ! U1 getvar "device.zuid"

```

**Values**
A unique hyphen-free string.


793


SGD Printer Commands
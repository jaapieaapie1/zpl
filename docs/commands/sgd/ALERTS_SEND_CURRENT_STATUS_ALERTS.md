# alerts.send_current_status_alerts



Generates all of the alerts specified in `alerts.configured` for the current status conditions in the
printer and sends the alerts to the specified destination. The destination can be one of the types specified
from `alerts.destinations` .


**Setvar**


To send the current status of the printer to the specified destination:

```
       ! U1 setvar "alerts.send_current_status_alerts" "<value>"

```

where <value> is `[destination],[destination_address],[port]` .

          - `[destination]` can be any of the values returned from `alerts.destination` .

          - `[destination_address]` applies to TCP, UDP, EMAIL, SNMP, SDK, MQTT, and HTTP POST
destination types.

          - `[port]` applies to TCP and UDP destination types.


**Example**

In this example the value of `alerts.configured` is:

```
       ! U1 setvar "alerts.configured" "COLD START,SNMP,Y,N,255.255.255.255,162,N,|
       ALL MESSAGES,MQTT,Y,Y,1,0,N,| HEAD OPEN,Y,N,255.255.255.255,162,N,"

```

Valid matches:

          - `"MQTT,1"` matches and would send alerts for all conditions to mqtt connection 1.

          - `"SNMP,255.255.255.255,162"` matches and would send a HEAD_OPEN alert if the head was open
by SNMP to 255.255.255.255 port 162.


Invalid matches:

          - `"MQTT,2"` does not match because the MQTT connection number is different.

          - " `SNMP,10.3.1.27,162"` does not match because the destination address is different.


624


SGD Printer Commands
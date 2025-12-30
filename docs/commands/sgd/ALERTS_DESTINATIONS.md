# alerts.destinations



This command lists the available destinations that can be specified in the first parameter of the
`alerts.add` SGD. See the `alerts.add` for information on the various parameters.


**Getvar**


To return a list of available alert destinations:

```
       ! U1 getvar "alerts.destinations"

```

**Values**
```
          SERIAL, PARALLEL, E-MAIL, TCP, UDP, SNMP, USB, HTTP-POST, BLUETOOTH,
          SDK, MQTT
```

**Default**


NA


613


SGD Printer Commands
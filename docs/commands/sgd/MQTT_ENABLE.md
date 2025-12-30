# mqtt.enable



This command controls MQTT functionality. When disabled, open MQTT connections are closed. When
enabled, a connection is established if all the configuration requirements are met.


**Setvar**


To enable or disable MQTT messaging:

```
       ! U1 setvar "mqtt.enable" "value"

```

**Values**

              - `"on"` enables MQTT messaging.

              - `"off"` disables MQTT messaging.

**Default**
```
          "off"

```

**Getvar**


To view whether MQTT messaging is enabled:

```
       ! U1 getvar "mqtt.enable"

```

**Values**

              - `"on"` indicates MQTT messaging is enabled.

              - `"off"` indicates MQTT messaging is disabled.


885


SGD Printer Commands
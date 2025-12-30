# device.syslog.enable



This printer setting enables syslog messages. The destination of syslog messages is specified in
`device.syslog.configuration` .


**Setvar**


To enable or disable syslog:

```
       ! U1 setvar "device.syslog.enable" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**


To retrieve if the syslog is enabled:

```
       ! U1 getvar "device.syslog.enable"

```

**Example**

This `setvar` example shows the value set to `"on"` .

```
       ! U1 setvar "device.syslog.enable" "on"

```

773


SGD Printer Commands
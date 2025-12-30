# ip.tls.enable



Enables the TLS connections to the encrypted channels on the printer specified by `ip.tls.port` and
```
       ip.tls.port_json_config.

```

**Setvar**


To set the command:

```
       ! U1 setvar "ip.tls.enable" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "on"

```

**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "ip.tls.enable"

```

**Example**


This example disables the TLS port.

```
       ! U1 setvar "ip.tls.enable" "off"

```

1326
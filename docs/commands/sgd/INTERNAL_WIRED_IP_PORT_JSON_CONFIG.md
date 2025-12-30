# internal_wired.ip.port_json_config



This command determines the TCP port number to listen on for JSON configuration packets.


**Setvar**


To set the TCP port number to listen on for JSON configuration packets:

```
       ! U1 setvar "internal_wired.ip.port_json_config" "value"

```

**Values**

              - `"0"` disables the port

              - `1-65535` Specifies the port number to listen on.

Ports that are already is use or the standard network ports are invalid values.#


**Default**
```
          "9200"

```

**Getvar**


To retrieve the port number:

```
       ! U1 getvar "internal_wired.ip.port_json_config"

```

**Example**


This example sets the port value to listen on as 1234.

```
       ! U1 setvar "internal_wired.ip.port_json_config" "1234"

```

1195


SGD Network Commands
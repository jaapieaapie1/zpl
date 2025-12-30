# wlan.ip.port_json_config



This command determines the TCP port number to listen on for JSON configuration packets.


**Setvar**


To determine the TCP port number on which to listen for JSON configuration packets:

```
       ! U1 setvar "wlan.ip.port_json_config" "value"

```

**Values**

`"0"` disables the port

`"1"` through `"65535"` for port number to listen on

**Default**
```
          "9200"

```

**Getvar**


To retrieve the TCP port number which is listening for JSON configuration packets:

```
       ! U1 getvar "wlan.ip.port_json_config"

```

**Example**

In this example, the `getvar` command causes the printer to get the TCP port number which is listening for
JSON configuration packets.

```
       ! U1 getvar "wlan.ip.port_json_config"

```

In this example, the `getvar` command causes the printer to get the TCP port number which is listening for
JSON configuration packets.

```
       ! U1 getvar "wlan.ip.port_json_config"

```

1431


SGD Network Commands
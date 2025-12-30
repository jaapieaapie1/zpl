# ip.port_json_config



This printer setting refers to the port number that the TCP print service is listening on for JSON
configuration packets. JSON TCP communications from the host should be directed to this port.


**Setvar**


To set the JSON TCP port number:

```
       ! U1 setvar "ip.port_json_config" "value"

```

**Values**

              - `"0"` disables the port

              - `"1"` through `"65535"` (excluding any ports currently used by other services, such as 21, 23, 80,
and 515).


**Default**
```
          "9200"

```

**Getvar**


To respond with the JSON TCP port number:

```
       ! U1 getvar "ip.port_json_config"

```

**Example**

This `setvar` example shows the value set to `"9200"` .

```
       ! U1 setvar "ip.port_json_config" "9200"

```

When the `setvar` value is set to `"9200"`, the `getvar` result is `"9200"` .


1306


SGD Network Commands
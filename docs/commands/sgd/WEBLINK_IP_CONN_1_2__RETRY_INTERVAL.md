# weblink.ip.conn[1|2].retry_interval



This command sets the number of seconds between attempts to connect to the server URL provided in
`weblink.ip.conn1.location` . If an attempt is unsuccessful or the connection is lost, the printer will
wait `'retry_interval'` seconds before attempting to connect again.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To set the number of seconds to wait before attempting to reconnect to the server:

```
       ! U1 setvar "weblink.ip.conn1.retry_interval" "value"

       ! U1 setvar "weblink.ip.conn2.retry_interval" "value"

```

**Values**

`"1"` through `"600"`

**Default**
```
          "10"

```

**Getvar**


To return the number of seconds to wait between connection attempts:

```
       ! U1 getvar "weblink.ip.conn1.retry_interval"

       ! U1 getvar "weblink.ip.conn2.retry_interval"

```

**Do**


To set the number of seconds to wait before attempting to reconnect to the server:

```
       ! U1 do "weblink.ip.conn1.retry_interval" "value"

       ! U1 do "weblink.ip.conn2.retry_interval" "value"

```

**Values**


1 - 600


**Default**
```
          "10"

```

1341


SGD Network Commands
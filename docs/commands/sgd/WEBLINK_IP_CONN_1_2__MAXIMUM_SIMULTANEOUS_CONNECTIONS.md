# weblink.ip.conn[1|2].maximum_simultaneous_connections



This command indicates the maximum number of simultaneous connections that can be initiated by the
printer.


Via the main connection (the original connection initiated by the printer to the remote server), the remote
server can request that additional connetions from the printer be initated (e.g. a connection that supports
only JSON SGDs, one that behaves similar to the RAW TCP port.


The server is free to request as many as it thinks it needs, but the printer will prevent more than N number
of connections, where N is the value of this command.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To set the maximum number of connections:

```
       ! U1 setvar "weblink.ip.conn1.maximum_simultaneous_connections" "value"

       ! U1 setvar "weblink.ip.conn2.maximum_simultaneous_connections" "value"

```

**Values**


Any integer from 1-100


**Default**
```
          "10"

```

**Getvar**


To retrieve the maximum set number of connections:

```
       ! U1 getvar "weblink.ip.conn1.maximum_simultaneous_connections"

       ! U1 getvar "weblink.ip.conn2.maximum_simultaneous_connections"

```

**Example**


This example sets the conn1 maximum connections to 3.

```
       ! U1 setvar "weblink.ip.conn1.maximum_simultaneous_connections" "3"

```

1338


SGD Network Commands
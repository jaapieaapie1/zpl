# weblink.ip.conn[1|2].num_connections



This command reports the number of established connections on conn1 and conn2. Once the connection
is established, this number will be incremented. When a channel is closed or the connection times out, the
number is decremented.


**Getvar**


To retrieve the number of active connections on conn1 and conn2:

```
       ! U1 getvar "weblink.ip.conn1.num_connections"

       ! U1 getvar "weblink.ip.conn2.num_connections"

```

1337


SGD Network Commands
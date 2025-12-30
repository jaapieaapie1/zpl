# ip.port_single_conn_idle_timeout



Sets the amount of time that must elapse with no activity for the single-connection IP port to be considered
in the idle state.


When the port is idle and a new connection is requested, the currently open connection will be closed
and a new connection will be opened. If the port is not in the idle state, the current connection will be
maintained and the connection request will be refused with an error response.


**Setvar**


To set the single connection timeout time:

```
       ! U1 setvar "ip.port_single_conn_idle_timeout" "value"

```

**Values**

A number between `"1"` and `"65535"` . The value is in seconds. If the value is `"0"`, the port will be
considered to be in the idle state.


**Default**
```
          "180"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "ip.port_single_conn_idle_timeout"

```

1308


SGD Network Commands
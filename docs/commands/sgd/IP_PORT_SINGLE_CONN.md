# ip.port_single_conn



This command configures the port number for the single connection IP port. The single-connection IP port
allows only one connection at a time.

See `"ip.port_single_conn_idle_timeout"` for additional information.


**Setvar**


To configure the port number for the single connection IP port:

```
       ! U1 setvar "ip.port_single_conn" "value"

```

**Values**

A number between `"1"` and `"65535"` . This number specifies the port.

**Default**
```
          "9300"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "ip.port_single_conn"

```

**Result**


A port number between 1 and 65535.


1307


SGD Network Commands
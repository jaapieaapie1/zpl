# ip.port_alternate



This command sets the port number of the alternate port.


**NOTE:** Print servers supporting this command will monitor both the primary port and the alternate
port for connections at the same time.


**Setvar**


To set the alternate port for the print server:

```
       ! U1 setvar "ip.port_alternate" "value"

```

**Values**


Any valid TCP port address.


**Default**
```
          "9100"

```

**Getvar**


To return the current alternate port setting:

```
       ! U1 getvar "ip.port_alternate"

```

**Values**


The current port setting.


**Example**

This `setvar` example shows the value set to `"6101"` .

```
       U1 setvar "ip.port_alternate" "6101"

```

1305


SGD Network Commands
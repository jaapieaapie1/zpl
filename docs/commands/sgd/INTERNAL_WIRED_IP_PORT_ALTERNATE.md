# internal_wired.ip.port_alternate



This command sets the port number of the alternate port.


**NOTE:** Print servers supporting this command will monitor both the primary port and the alternate
port for connections at the same time.


**Setvar**


To set the alternate port for the print server:

```
       ! U1 setvar "internal_wired.ip.port_alternate" "value"

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
       ! U1 setvar "internal_wired.ip.port_alternate"

```

1194


SGD Network Commands
# netmanage.avalanche.tcp_connection_timeout



This command sets the Network Management Timeout used for establishing a TCP connection to an
Agent. Time is set in milliseconds.


**Setvar**


To set the Network Management Timeout used for establishing a TCP connection to an Agent:

```
       ! U1 setvar "netmanage.avalanche.tcp_connection_timeout" "value"

```

**Values**

Any integer value from `"0"` to `"4294967295"`

**Default**
```
          "0"

```

**Getvar**


To retrieve the current Network Management Timeout used for establishing a TCP connection to an Agent:

```
       ! U1 getvar "netmanage.avalanche.tcp_connection_timeout"

```

**Example**


This examples sets the connection timeout to 2000 milliseconds (2 seconds).

```
       ! U1 setvar "netmanage.avalanche.tcp_connection_timeout" "2000"

```

908


SGD Printer Commands
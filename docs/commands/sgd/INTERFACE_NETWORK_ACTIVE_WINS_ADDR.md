# interface.network.active.wins_addr



This command sets and retrieves the active WINS address.


**Setvar**


To set the WINS address of the active print server:

```
       ! U1 setvar "interface.network.active.wins_addr" "value"

```

**Values**
```
          "0.0.0.0" - "255.255.255.255"

```

**Getvar**


To retrieve the WINS address of the active print server:

```
       ! U1 getvar "interface.network.active.wins_addr"

```

**Result**


A WINS address.


**Example**

In this example, the `setvar` sets the Wins address of the active print server.

```
       ! U1 setvar "interface.network.active.wins.addr" "10.3.5.120"

```

1155


SGD Network Commands
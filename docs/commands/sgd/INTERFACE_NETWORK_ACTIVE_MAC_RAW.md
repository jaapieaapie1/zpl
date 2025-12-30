# interface.network.active.mac_raw



This command identifies the RAW MAC address of the active print server. The raw mac address is the mac
address without the colons ( `":"` ).


**Getvar**


To retrieve the RAW MAC address of the active print server:

```
       ! U1 getvar "interface.network.active.mac_raw"

```

**Example**

In this example, the `getvar` retrieves the RAW MAC address of the active print server.

```
       ! U1 getvar "interface.network.active.mac_raw"
       "00074d2408ff"

```

1145


SGD Network Commands
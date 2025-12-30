# interface.network.active.arp_interval



This command changes the arp interval.


**Setvar**


To retrieve the gateway address of the active print server:

```
       ! U1 getvar "interface.network.active.arp_interval" "value"

```

**Values**

Integer values from `"0"` to `"30"`

**Default**
```
          "0"

```

**Getvar**


To retrieve the current arp interval setting, shown in minutes:

```
       ! U1 getvar "interface.network.active.arp_interval"

```

**Example**

In this example, the `setvar` changes the arp interval to three minutes.

```
       ! U1 getvar "interface.network.active.arp_interval" "3"

```

1135


SGD Network Commands
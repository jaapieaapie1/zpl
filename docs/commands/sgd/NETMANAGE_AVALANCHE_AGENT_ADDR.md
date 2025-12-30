# netmanage.avalanche.agent_addr



This parameter obtains or changes the Network Management agent IP address.


**Setvar**


To set the Network Management Agent IP address:

```
       ! U1 setvar "netmanage.avalanche.agent_addr" "value"

```

**Values**


Any valid IP address.


**Default**
```
          "0.0.0.0"

```

**Getvar**


To retrieve the current Network Management IP address:

```
       ! U1 getvar "netmanage.avalanche.agent_addr"

```

**Example**

```
       ! U1 setvar "netmanage.avalanche.agent_addr" "10.14.2.200"

```

899


SGD Printer Commands
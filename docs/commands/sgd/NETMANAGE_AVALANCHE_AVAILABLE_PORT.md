# netmanage.avalanche.available_port



This command returns the available port of the remote agent found during the Agent Discovery Phase.


**Setvar**


To set the available port of the remote agent found during the Agent Discovery Phase:

```
       ! U1 setvar "netmanage.avalanche.available_port" "value"

```

**Values**

`"0"` to to `"65535"`

**Default**
```
          "0"

```

**Getvar**


To retrieve the current port setting of the remote agent found during the Agent Discovery Phase:

```
       ! U1 getvar "netmanage.avalanche.available_port"

```

**Example**

```
       ! U1 setvar "netmanage.avalanche.available_port" "1800"

```

901


SGD Printer Commands
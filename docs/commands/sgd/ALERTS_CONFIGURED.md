# alerts.configured



This command creates a list of all the alerts that are configured on the printer. The alerts are delimited by
the `'|'` character.


**NOTE:** Writing to this SGD will clear out the old alerts and set up the new ones.


**Setvar**


To create the list of alerts configured on the printer:

```
       ! U1 setvar "alerts.configured" "<a '|' delimited list of configured alerts>"

```

**Values**

A list of alerts to be set up on the printer. See `alerts.add` for the format of the individual alerts.

**Default**
```
          "COLD START,SNMP,Y,N,255.255.255.255,162,N"

```

**Getvar**


To retrieve the currently configured alerts on the printer:

```
       ! U1 getvar "alerts.configured"

```

612


SGD Printer Commands
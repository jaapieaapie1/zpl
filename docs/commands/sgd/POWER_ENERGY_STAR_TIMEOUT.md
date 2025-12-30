# power.energy_star.timeout



Sets the amount of idle time before Energy Star mode in invoked. The time is specified is in seconds.


For more information on Energy Star, see [http://www.energystar.gov.](http://www.energystar.gov)


**Setvar**


To set the amount of idle time before Energy Star mode in invoked:

```
       ! U1 setvar "power.energy_star.timeout" "value"

```

**Values**

`"180"` to `"65535"` seconds

**Default**
```
          "180"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "power.energy_star.timeout"

```

**Example**

This `setvar` example shows the value set to `"260"` .

```
       ! U1 setvar "power.energy_star.timeout" "260"

```

The `setvar` value is the `getvar` result. In this example, the `getvar` result is `"260"` .


948


SGD Printer Commands
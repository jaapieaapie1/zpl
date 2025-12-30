# ip.mirror.freq



This command defines the frequency of mirroring updates (in minutes).


**Setvar**


To set the mirror frequency:

```
       ! U1 setvar "ip.mirror.freq" "value"

```

**Values**

`"0"` through `"65535"` (minutes)

**Default**

`"0"` (disables this feature)


**IMPORTANT:** When the `"ip.mirror.freq"` is set to a low value (other than zero) the
printer will spend a lot of time performing the mirroring process.


**Getvar**


To retrieve the number of minutes to wait before performing another mirror update:

```
       ! U1 getvar "ip.mirror.freq"

```

**Example**

This `setvar` example shows the value set to `"1000"` .

```
       ! U1 setvar "ip.mirror.freq" "1000"

```

When the `setvar` value is set to `"1000"`, the `getvar` result is `"1000"` and mirroring will be attempted
every 1000 minutes.


1276


SGD Network Commands
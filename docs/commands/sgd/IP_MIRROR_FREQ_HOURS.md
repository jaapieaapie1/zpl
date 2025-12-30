# ip.mirror.freq_hours



This command defines the frequency of mirroring updates (in hours).


**Setvar**


To set the frequency of mirroring updates (in hours):

```
       ! U1 setvar "ip.mirror.freq_hours" "values"

```

**Values**

`"0"` through `"100"`

**Default**

`"0"` (disables this feature)


**Getvar**


To retrieve the frequency of mirroring updates (in hours) that the printer is currently using:

```
       ! U1 getvar "ip.mirror.freq_hours"

```

**Example**

This `setvar` example shows the value set to `"8"` .

```
       ! U1 setvar "ip.mirror.freq_hours" "8"

```

When the `setvar` value is set to `"8"`, the `getvar` result is `"8"` and mirroring will be attempted every 8
hours.


1277


SGD Network Commands
# ip.mirror.auto



This command enables the ability to automatically perform a mirror update (fetch) command on power up.


**Setvar**


To perform a mirror update (fetch) command when the printer is turned on using the interval that is set for
`"ip.mirror.freq"` or `"ip.mirror.freq_hours":`

```
       ! U1 setvar "ip.mirror.auto" "values"

```

**Values**

              - `"on"` turns on the auto mirroring feature

              - `"off"` turns off the auto mirroring feature

**Default**
```
          "off"

```

**Getvar**


To report whether the printer will perform a mirror update (fetch) automatically on power up:

```
       ! U1 getvar "ip.mirror.auto"

```

**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "ip.mirror.auto" "off"

```

When the `setvar` value is set to `"off"`, the `getvar` result is `"off"` .


1269


SGD Network Commands
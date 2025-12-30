# ip.mirror.feedback.auto



This command identifies if a feedback file is pushed to the mirroring server by the printer when a mirroring
update (fetch) is complete.


**Setvar**


To set the mirror feedback feature to on or off:

```
       ! U1 setvar "ip.mirror.feedback.auto" "value"

```

**Values**

`"on"` turns on mirror feedback

`"off"` turns off mirror feedback

**Default**
```
          "off"

```

**Getvar**


To return the current setting value:
```
       ! U1 getvar "ip.mirror.feedback.auto"

```

**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "ip.mirror.feedback.auto" "off"

```

When the `setvar` value is set to `"off"`, the `getvar` result is `"off"` .


1271


SGD Network Commands
# weblink.ip.conn1.retry_interval_random_max



Specifies the maximum random wait time in seconds for weblink connection retries.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To set the command:

```
       ! U1 setvar "weblink.ip.conn[1|2].retry_interval_random_max" "value"

```

**Values**

`"0"` through `"600"`

**Default**
```
          "120"

```

**Examples**

```
       ! U1 setvar "weblink.ip.conn[1].retry_interval_random_max" "120"

       ! U1 setvar "weblink.ip.conn[2].retry_interval_random_max" "60"

```

**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "weblink.ip.conn[1|2].retry_interval_random_max"

```

1345


SGD Network Commands
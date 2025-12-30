# weblink.ip.conn[1|2].test.retry_interval



This command determines how often, in seconds, a connection to the test.location URL should be
attempted. This setting is only applicable when the `test.test_on` SGD is set to `"interval"` or
`"both"` .

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To set the interval for how often a connection to the test.location URL should be attempted:

```
       ! U1 setvar "weblink.ip.conn1.test.retry_interval" "value"

       ! U1 setvar "weblink.ip.conn2.test.retry_interval" "value"

```

**Values**


0-1800 (in seconds, providing 0 second - 30 minute interval)


**Default**
```
          "900"

```

**Getvar**


To retrieve the retry interval:

```
       ! U1 getvar "weblink.ip.conn1.test.retry_interval"

       ! U1 getvar "weblink.ip.conn2.test.retry_interval"

```

**Do**


To set the interval for how often a connection to the test.location URL should be attempted:

```
       ! U1 do "weblink.ip.conn1.test.retry_interval" "value"

       ! U1 do "weblink.ip.conn2.test.retry_interval" "value"

```

**Values**


0-1800 (in seconds, providing 0 second - 30 minute interval)


**Default**
```
          "900"

```

1344


SGD Network Commands
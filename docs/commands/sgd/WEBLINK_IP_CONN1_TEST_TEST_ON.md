# weblink.ip.conn1.test.test_on



This command determines when the test connection should be attempted. This assists in debugging the
printer's connection to remote servers when there are issues with the main weblink connection (conn1 or
conn2).

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To indicate when the test connection should be attempted:

```
       ! U1 setvar "weblink.ip.conn1.test.test_on" "value"

       ! U1 setvar "weblink.ip.conn2.test.test_on" "value"

```

**Values**
```
          off,failure,interval,both
```

**Default**
```
          "failure"

```

**Getvar**


To retrieve the test connection setting:

```
       ! U1 getvar "weblink.ip.conn1.test.test_on"

       ! U1 getvar "weblink.ip.conn2.test.test_on"

```

**Do**


To set when the test connection should be attempted:

```
       ! U1 do "weblink.ip.conn1.test.test_on" "value"

       ! U1 do "weblink.ip.conn2.test.test_on" "value"

```

**Values**
```
          off,failure,interval,both
```

**Default**
```
          "failure"

```

**Example**


The test connection can assist the user is several ways/scenarios:

If the `test.test_on` value is set to `"failure"`, any time the main weblink ( `conn[1|2].location` )
connection fails to connect then the `test.location` URL will be used. An attempt will be made to


1346


SGD Network Commands


contact the remote URL in `test.location`, using authentication and proxy configuration that is specified
by the main connection.

If the `test.test_on` value is set to `"interval"` an attempt will be made to contact the remote URL in
`test.location` every `test.retry_interval` seconds, using authentication and proxy configuration
that is specified by the main connection.

If the `test.test_on` value is set to `"both"`, then scenario 1 and 2 will both occur. This is useful for users
who will use an HTTP connection to move through their firewall - and thereafter frequently refresh the
connection to indicate to their firewall that there is still activity for the purpose of keeping the connection
alive.


1347


SGD Network Commands
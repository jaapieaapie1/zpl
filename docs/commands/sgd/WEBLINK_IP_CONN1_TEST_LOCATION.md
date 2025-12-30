# weblink.ip.conn1.test.location



This command holds the URL for testing a connection to the internet. This is meant to assist users in
debugging their printer's connection to remote servers when there are issues with the main weblink
connection (conn1 or conn2).


The URL must follow the URL rules for the HTTP[S] protocol outlined in RFC2396 (http://www.ietf.org/rfc/
rfc2396.txt).

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To set the URL to hold for testing a connection:

```
       ! U1 setvar "weblink.ip.conn1.test.location" "url"

       ! U1 setvar "weblink.ip.conn2.test.location" "url"

```

**Values**


Any HTTPS URL up to 2048 characters


**Default**
```
          "http://www.zebra.com/apps/linktest"

```

**Getvar**


To retrieve the printer's test connection URL:

```
       ! U1 getvar "weblink.ip.conn1.test.location"

       ! U1 getvar "weblink.ip.conn2.test.location"

```

**Do**


To set the URL to hold for testing a connection:

```
       ! U1 do "weblink.ip.conn1.test.location" "url"

       ! U1 do "weblink.ip.conn2.test.location" "url"

```

**Values**


Any HTTPS URL up to 2048 characters


**Default**
```
          "http://www.zebra.com/apps/linktest"

```

**Example**


The test connection can assist the user in several ways/scenarios:


1342


SGD Network Commands


If the `test.test_on` value is set to `"failure"`, any time the main weblink ( `conn[1|2].location` )
connection fails to connect then the `test.location` URL will be used. In this situation, an attempt will be
made to contact the remote URL in `test.location`, using authentication and proxy configuration that is
specified by the main connection.

If the `test.test_on` value is set to `"interval"` an attempt will be made to contact the remote URL in
`test.location` every `test.retry_interval` seconds, using authentication and proxy configuration
that is specified by the main connection.

If the `test.test_on` value is set to `"both"`, then scenario 1 and 2 will both occur. This is useful for users
who will use an HTTP connection to move through their firewall - and thereafter frequently refresh the
connection to indicate to their firewall that there is still activity for the purpose of keeping the connection
alive.


1343


SGD Network Commands
# weblink.ip.conn[1|2].location



This command assigns the URL of the server for this connection. The URL must follow the URL rules for the
HTTP[S] protocol outlined in RFC2396 (http://www.ietf.org/rfc/rfc2396.txt).


The setting will not take effect until the printer is reset. Changing this setting will set
weblink.printer_reset_required to `"yes"` .

`^JUF`, `^JUS`, `^JUN`, `^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To set the server URL for the specified connection:

```
       ! U1 setvar "weblink.ip.conn1.location" "value"

       ! U1 setvar "weblink.ip.conn2.location" "value"

```

**Values**


Any HTTPS URL up to 2048 characters


**Default**
```
          ""

```

**Getvar**


To return the server URL currently assigned to the connection:

```
       ! U1 getvar "weblink.ip.conn1.location"

       ! U1 getvar "weblink.ip.conn2.location"

```

**Example**

```
       ! U1 setvar "weblink.ip.conn2.location"

       "https://my.linkos.server.com:8080/link/os"

```

1336


SGD Network Commands
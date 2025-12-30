# weblink.ip.conn[1|2].authentication.entries



This command lists the server names added to the authentication entries list.


Only the server names will be shown. The username and passwords will not be shown.The server names
are separated by a `\r\n` so that each shows up on its own line.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Getvar**


To list the server names for the specified connection:

```
       ! U1 getvar "weblink.ip.conn1.authentication.entries"

       ! U1 getvar "weblink.ip.conn2.authentication.entries"

```

1334


SGD Network Commands
# weblink.ip.conn[1|2].authentication.add



This command allows the user to add a single server/username/password triplet into the list of
authentication entries.

When the printer attempts to connect to the server (url specified in `weblink.ip.conn[1|`
`2].location` ) the server may require HTTP authentication (e.g. digest, basic, DNS, etc.). There may be
multiple authentication requests along the route to the destination (e.g. a locals erver first requires HTTP
authentication as well as on the remote server).


For each HTTP authentication request received while attempting to connect, the printer will enumerate the
authentication entries and attempt to satisfy the request with the username/password pair provided for the
respective server. The server name in the entry is what determines which username/password pair should
be used for which authentication request. Both DNS names and IP addresses are acceptable. The server,
username, and password are separated by a single space (not a tab or other white space character). The
server name is the only required field. If no username is supplied, but a password is, there must be two
spaces between the server and the password fields. If there is a username but no password, or simply just
the servername, no space is required at the end of the entry.

If the command is changed when the connection is enabled ( `weblink.enable is set to "on"` ), it
will not take effect until the connection is disabled, and then re-enabled.


**NOTE:** This setting only be changed when `weblink.enable` is set to `"off"` .


`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To add a single server/username/password triplet to the list of authentication entries:

```
       ! U1 setvar "weblink.ip.conn1.authentication.add" "servername[ username]
       [ password]"

       ! U1 setvar "weblink.ip.conn2.authentication.add" "servername[ username]
       [ password]"

```

**Values**

`"servername [username][ password]"` has a maximum length of 2048 characters

**Default**


NA


**Example**


In this example, a username and a password is supplied:

```
       ! U1 setvar "weblink.ip.conn1.authentication.add" "my.server.lan johndoe
       password"

```

In this example, no password is supplied


1332


SGD Network Commands

```
! U1 setvar "weblink.ip.conn1.authentication.add" "my.server.lan johndoe"

```

In this example, no username is supplied (note the double space)

```
! U1 setvar "weblink.ip.conn1.authentication.add" "my.server.lan password"

```

In this example, no username or password is supplied

```
! U1 setvar "weblink.ip.conn1.authentication.add" "my.server.lan"

```

1333


SGD Network Commands
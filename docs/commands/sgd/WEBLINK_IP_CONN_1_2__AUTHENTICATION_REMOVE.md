# weblink.ip.conn[1|2].authentication.remove



This command allows the user to remove a single server/username/password triplet from the list of
authentication entries.


To remove an entry only the server name is supplied. If an invalid entry is supplied no action is taken. If
the SGD is changed when the connection is enabled ( `weblink.ip.conn[1|2].enable` ), it will not take
effect until the connection is disabled, and then re-enabled. It is therefore recommended that this setting
only be changed when `weblink.ip.conn[1|2].enable` is set to `"off"` .

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To remove a single server/username/password triplet from the list of authenticaiton entries:

```
       ! U1 setvar "weblink.ip.conn1.authentication.remove" "servername"

       ! U1 setvar "weblink.ip.conn2.authentication.remove" "servername"

```

**Values**

`"servername"` has a maximum length of string is 2048 characters.

**Default**


NA


**Example**


In this example, a username and a password is supplied

```
       ! U1 setvar "weblink.ip.conn1.authentication.remove" "my.server.lan"

```

1335


SGD Network Commands
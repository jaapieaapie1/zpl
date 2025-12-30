# weblink.zebra_connector.authentication.entries



This command lists the server names added to the authentication entries list.


Only the server names will be shown. The username and passwords will not be shown.The server names
are separated by a `\r\n` so that each shows up on its own line.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Getvar**


To list the server names added to the authentication entries list:

```
       ! U1 getvar "weblink.zebra_connector.authentication.entries"

```

**Result**


Returns the list of servers with authentication entries. It does not return the username or passwords
for those servers.


**Default**
```
          ""

```

1355


SGD Network Commands
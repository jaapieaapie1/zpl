# weblink.zebra_connector.authentication.add



This command allows the user to add a single server/username/password triplet into the list of
authentication entries.


When the printer attempts to connect to the Zebra Printer Connector, the local server may require HTTP
authentication (e.g. digest, basic, DNS, etc.). There may be multiple authentication requests along the route
to the destination (e.g. a local server first requires HTTP authentication as well as on the remote server).


For each HTTP authentication request received while attempting to connect, the printer will enumerate the
authentication entries and attempt to satisfy the request with the username/password pair provided for the
respective server. The server name in the entry is what determines which username/password pair should
be used for which authentication request. Both DNS names and IP addresses are acceptable. The server,
username, and password are separated by a single space (not a tab or other white space character). The
server name is the only required field. If no username is supplied, but a password is, there must be two
spaces between the server and the password fields. If there is a username but no password, or simply just
the servername, no space is required at the end of the entry.


If the command is changed while the Visibility Agent is enabled, it will not take effect until the connection is
disabled, and then re-enabled.


**IMPORTANT:** This setting can only be changed when weblink.zebra_connector.enable is set to
`"off"` .

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To allow the user to add a single server/username/password triplet into the list of authentication entries:

```
       ! U1 setvar "weblink.zebra_connector.authentication.add" "server username
       password"

```

**Values**

              - `"server"` is the IP address or a DNS name

              - `"username"` is the user name on this server

              - `"password"` is the password for this username on this server

**Default**
```
          ""
```

**Result**

```
          ! U1 setvar "weblink.zebra_connector.authentication.add"

          "10.3.5.70 jsmith LedZepR0cks!"

```

1354


SGD Network Commands
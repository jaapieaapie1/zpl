# ip.firewall.authentication.add



This command allows a user to add a single server/username/password triplet into the list of authentication
entries. This authentication entry is applied before making an outgoing HTTP/HTTPS connection in case
the printer must go through an authentication server beforehand. This setting is separate from the proxy
setting.


The server, username, and password parameters are separated by a space, not a tab or other white space
character. The server name is required. If no username is supplied, but a password is, there must be two
spaces between the server and the password fields. If there is a username but no password, or simply
just the server name, no space is required at the end of the entry. Both DNS names and IP addresses are
acceptable for the server name.


**Setvar**


To add a server, username, and password to the list of authentication entries:

```
       U1 setvar "ip.firewall.authentication.add" "servername[ username][ password]"

```

**Values**

              - `servername` is required. DNS or IP address is acceptable.

              - `username` is an optional value. When a username is provided, it must be separated from the
server name by a space.

              - `password` is an optional value. When a password is provided, it must be separated from the
username by a space. If the username is omitted, the password must be separated from the
servername by two spaces.


The maximum length of the authentication entry is up to 1024 characters.


1246


SGD Network Commands
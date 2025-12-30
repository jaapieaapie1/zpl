# alerts.http.authentication.add



This command allows the user to add a single server/username/password triplet into the list of
authentication entries.


When the printer attempts to connect to the URL in the HTTP POST alert, the server may require HTTP
authentication (such as digest, basic, DNS, etc.). There may be multiple authentication requests along the
route to the destination (for example, a local server first requires HTTP authentication as well as on the
remote server). For each HTTP authentication request received while attempting to connect, the printer
will enumerate the authentication entries and attempt to satisfy the request with the username/password
pair provided for the respective server. The server name in the entry is what determines which username/
password pair should be used for which authentication request. Both DNS names and IP addresses are
acceptable.


The server, username, and password are separated by a single space (not a tab or other white space
character). The server name is the only required field. If no username is supplied, but a password is, there
must be two spaces between the server and the password fields. If there is a username but no password,
or simply just the servername, no space is required at the end of the entry.


**Setvar**


To add server/username/password triplet into the list of authentication entries:

```
       ! U1 setvar "alerts.http.authentication.add" "servername[ username]
       [ password]"

```

**Values**


Maximum string of 2048 characters.


**Default**


NA


**Do**

This command has the same settings as the `setvar` .

To add server/username/password triplet into the list of authentication entries:

```
       ! U1 do "alerts.http.authentication.add" "servername[ username][ password]"

```

**Values**


Maximum string of 2048 characters.


**Default**


NA


**Example 1**


A username and a password is supplied:

```
       ! U1 setvar "alerts.http.authentication.add" "my.server.lan johndoe password"

```

614


SGD Printer Commands


**Example 2**


No password is supplied:

```
! U1 setvar "alerts.http.authentication.add" "my.server.lan johndoe"

```

**Example 3**


No username is supplied (note the double space):

```
! U1 setvar "alerts.http.authentication.add" "my.server.lan password"

```

**Example 4**


No username or password is supplied:

```
! U1 setvar "alerts.http.authentication.add" "my.server.lan"

```

615


SGD Printer Commands
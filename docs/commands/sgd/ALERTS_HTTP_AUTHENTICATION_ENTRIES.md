# alerts.http.authentication.entries



This command lists the server names added to the authentication entries list via
```
       alerts.http.authentication.add.
```

Only the server names will be shown; the username and passwords will not be shown. The server names
are separated by a `\r\n` so that each shows up on its own line and is easier to read.


**Getvar**


To return the server names added to the authentication entry list:

```
       ! U1 getvar "alerts.http.authentication.entries"

```

**Values**


A list of server names.


**Default**


NA


616


SGD Printer Commands
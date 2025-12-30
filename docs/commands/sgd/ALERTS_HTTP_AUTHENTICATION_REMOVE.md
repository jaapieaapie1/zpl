# alerts.http.authentication.remove



This command allows the user to remove a single server/username/password triplet from the list of
authentication entries. To remove an entry, only the server name is supplied, and the entire entry will be
removed. If an invalid entry is supplied no action is taken.


Note that the list of authentication triplets will be updated (and saved over a reset) but this SGD is just a
command and doesn't have state. Therefore the persistent and restore defaults do not apply. The internal
list that this command removes from, however, is persistent and defaultable (defaults to an empty list).


**Setvar**


To remove a server/username/password triplet from the list of authentication entries:

```
       ! U1 setvar "alerts.http.authentication.remove" "servername"

```

**Value**


Maximum string of 2048 characters


**Default**


NA


**Do**

This command has the same settings as the `setvar` .

To remove a server/username/password triplet from the list of authentication entries:

```
       ! U1 do "alerts.http.authentication.remove" "servername"

```

**Value**

```
          Maximum string of 2048 characters

```

**Default**


NA


**Example**


A username and a password is supplied

```
       ! U1 setvar "alerts.http.authentication.remove" "my.server.lan"

```

617


SGD Printer Commands
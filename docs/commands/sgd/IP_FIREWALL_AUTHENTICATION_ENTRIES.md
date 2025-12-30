# ip.firewall.authentication.entries



This command retrieves a list of the server names added to the authentication entries list. The list includes
only the server names, the usernames and passwords are not shown.


**Getvar**


To have the printer return a list of server names on the authentication entries list:

```
       ! U1 getvar "ip.firewall.authentication.entries"

```

**Result**
The list of server names on the authentication entries list. A carriage return line feed delimits server
names.


1247


SGD Network Commands
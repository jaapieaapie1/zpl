# ip.firewall.authentication.remove



This command allows a user to remove a single server/username/password triplet from the list of
authentication entries. Only the servername is required to remove a complete entry. If an invalid entry is
supplied, no action is taken.


**Setvar**


To remove an entry from the list of authentication entries:

```
       ! U1 setvar "ip.firewall.authentication.remove" "value"

```

**Value**
`"value"` is the server name. A DNS name or IP address is acceptable.


1248


SGD Network Commands
# weblink.zebra_connector.authentication.remove



Removes a single server/username/password triplet from the list of authentication entries.


To remove an entry only the server name is supplied, however the entire entry will be removed. If an
invalid entry is supplied no action is taken.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To remove a single server/username/password triplet from the list of authentication entries:

```
       ! U1 setvar "weblink.zebra_connector.authentication.remove" "server"

```

**Values**

`"server"` an IP address or a DNS name of the server to remove

**Default**


NA


**Example**

```
       ! U1 setvar "weblink.zebra_connector.authentication.remove" "10.3.5.70"

```

1356


SGD Network Commands
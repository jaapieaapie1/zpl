# weblink.logging.entries



This command returns the N number of entries in the weblink log, where N has a maximum value that is set
by `weblink.logging.max_entries` .

The weblink log is a collection of events related to connecting to a remote Link-OS™ server. The log entries
range anywhere from general status to errors that prevented a successful connection. The log contains
entries from all connections and are labeled so that it is clear which log entries are for which connection.
Each log entry also contains a timestamp for when it was logged by the system. The newest events will
appear at the bottom of the list.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Getvar**


To return a lists of entries in the weblink log:

```
       ! U1 getvar "weblink.logging.entries"

```

**Values**


NA


**Default**
```
          ""

```

**Example**

This example shows the result from `weblink.logging.entries:`

```
       [01-04-2013 08:40:45.655] [conn1.1] HTTP/1.1 404 Not Found
       [01-04-2013 08:40:45.659] [conn1.1] Received HTTP code 404 from proxy after
       CONNECT
       [01-04-2013 08:40:45.660] [conn1.1] Closing connection
       [01-04-2013 08:40:45.662] [conn1.1] Failed to connect (SP = 0, CU = 0, UW =
       0, AC = 0, PC = 0)

```

1349


SGD Network Commands
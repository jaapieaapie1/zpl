# weblink.logging.max_entries



This command specifies the maximum number of individual log entries that will be stored in the
`weblink.logging.entries` command.


**NOTE:** Changes to this command are immediate and may result in some log entries being lost.
If there are N log entries currently in the log, the user sets the max_entires to M, where M is less
than N, the oldest (N-M) log entries will be removed.

`^JUF, ^JUS, ^JUN, ^JUA`, and `device.restore_defaults` do not have any affect on this setting.


**Setvar**


To set the maximum number of log entires that will be stored:

```
       ! U1 setvar "weblink.logging.max_entries" "value"

```

**Values**

`"0"` through `"10000"`

Setting the value to `"0"` disables logging.


**Getvar**


To return the setting for the maximum number of log entries that will be stored:

```
       ! U1 getvar "weblink.logging.max_entries"

```

**Do**


To set the maximum number of log entires that will be stored:

```
       ! U1 getvar "weblink.logging.max_entries"

```

**Values**

`"0"` through `"10000"`

Setting the value to `"0"` disables logging.

**Default**
```
          "0"

```

**Example**

In this example, `weblink.logging.max_entries` is set to 3:

```
       [01-04-2013 08:40:45.659] [conn1.1] Received HTTP code 404 from proxy after
       CONNECT
       [01-04-2013 08:40:45.660] [conn1.1] Closing connection
       [01-04-2013 08:40:45.662] [conn1.1] Failed to connect (SP = 0, CU = 0, UW =
       0, AC = 0, PC = 0)

```

In this example, `weblink.logging.max_entries` is set to 2: `weblink.logging.entries` becomes:


1350




SGD Network Commands

```
[01-04-2013 08:40:45.660] [conn1.1] Closing connection
[01-04-2013 08:40:45.662] [conn1.1] Failed to connect (SP = 0, CU = 0, UW =
0, AC = 0, PC = 0)

```

1351


SGD Network Commands
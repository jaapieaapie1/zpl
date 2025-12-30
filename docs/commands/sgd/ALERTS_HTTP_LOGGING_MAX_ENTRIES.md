# alerts.http.logging.max_entries



This command specifies the maximum number of individual log entries that will be stored in the
`alerts.http.logging.entries` command.


**NOTE:** Changes to this command are immediate and may result in some log entries being lost.
If there are N log entries currently in the log, the user sets the max_entires to M, where M is less
than N, the oldest (N-M) log entries will be removed.


**Setvar**


To set the maximum number of log entries that will be stored:

```
       ! U1 getvar "alerts.http.logging.max_entries" "value"

```

**Values**

`"0"`         - `"10000"`


**NOTE:** Setting the value to 0 disables logging.


**Default**
```
          "0"

```

**Getvar**


To return the setting for the maximum number of log entries that will be stored:

```
       ! U1 getvar "alerts.http.logging.max_entries"

```

**Do**


To set the maximum number of log entries that will be stored:

```
       ! U1 do "alerts.http.logging.max_entries" "value"

```

**Values**

`"0"`         - `"10000"`


**NOTE:** Setting the value to 0 disables logging.


**Default**
```
          "0"

```

**Example**

In this example, `alerts.http.logging.max_entries` is set to 2.

The original log file:

```
       [01-03-2013 12:48:59.964] [http] Connected to 10.3.4.58 (10.3.4.58) port 80
       [01-03-2013 12:48:59.978] [http] HTTP/1.1 100 Continue

```

620




SGD Printer Commands

```
[01-03-2013 12:49:01.999] [http] Closing connection

```

When `alerts.http.logging.max.entries` is set to `"2"`, the log file is:

```
[01#03#2013 12:48:59.978] [http] HTTP/1.1 100 Continue
[01#03#2013 12:49:01.999] [http] Closing connection

```

621


SGD Printer Commands
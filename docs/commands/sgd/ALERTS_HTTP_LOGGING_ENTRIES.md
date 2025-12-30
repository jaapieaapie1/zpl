# alerts.http.logging.entries



This command returns the N number of entries in the http log, where N has a maximum value that is set by
`alerts.http.logging.max_entries` .

The alerts http log is a collection of events related to sending HTTP POST messages. The log entries range
anywhere from general status to errors that prevented a successful connection. Each log entry contains a
timestamp for when it was logged by the system. The newest events will appear at the bottom of the list.


**Getvar**


To return the number of entries in the HTTP log:

```
       ! U1 getvar "alerts.http.logging.entries"

```

**Values**


NA


**Default**


NA


**Example**

This example shows the result from `alerts.http.logging.entries` :

```
       [01-03-2013 12:48:59.964] [http] Connected to 10.3.4.58 (10.3.4.58) port 80
       [01-03-2013 12:48:59.978] [http] HTTP/1.1 100 Continue
       [01-03-2013 12:49:01.999] [http] Closing connection

```

619


SGD Printer Commands
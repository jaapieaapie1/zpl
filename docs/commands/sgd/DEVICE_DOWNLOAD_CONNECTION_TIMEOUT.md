# device.download_connection_timeout



This command instructs the printer to abort a firmware download if the printer fails to receive any
download data in the set amount of seconds. If the set amount of seconds is exceeded, the download will
be aborted, and the printer automatically restarts. This command prevents the printer from being locked
into the downloading state, if the communication to the host is interrupted.


**Setvar**


To instruct the printer to abort a firmware download if the printer fails to receive any download data in the
set amount of seconds:

```
       ! U1 setvar "device.download_connection_timeout" "value"

```

**Values**

`"0"` through `"65535"`

**Default**

`"0"` ( `"0"` disables this feature)


**Getvar**


To retrieves the connection time out value (in seconds):

```
       ! U1 getvar "device.download_connection_timeout"

```

**Example**

This `setvar` example shows the value set to `"0"` .

```
       ! U1 setvar "device.download_connection_timeout" "0"

```

When the `setvar` value is set to `"0"`, the `getvar` result is `"0"` .


684


SGD Printer Commands
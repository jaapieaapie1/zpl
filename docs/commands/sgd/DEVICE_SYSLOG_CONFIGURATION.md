# device.syslog.configuration



This setting specifies the location for the syslog messages to be recorded. The location may be either on
the printer, or a syslog server IP address.


**Setvar**


To specify the location for the syslog messages to be recorded:

```
       ! U1 setvar "device.syslog.configuration" "value"

```

**Values**


A list of configuration entries, limited to 1000 characters. Entries must be in the form of
`"severity,destination"` and delimited with a semi-colon.

SEVERITY - The severity levels, in decreasing severity order:


              - emerg


              - alert


              - crit


              - err


              - warning


              - notice


              - info


              - debug


When you specify the severity level, the lowest specified severity and all severity levels above it will
be recorded. For example, if you specify `debug`, you will get all severity level reports. If you specify
`crit`, you will get only `crit`, `alert`, and `emerg` severity reports.

DESTINATION - `"local"` or a syslog server IP address

When configuring the local syslog report, the first local entry is used and duplicate requests to local
are ignored. To configure remote syslog messages you will first need a syslog server to accept
them.


**Default**
```
          ""

```

**Getvar**


To retrieve the configuration string setting:

```
       ! U1 getvar "device.syslog.configuration"

```

**Example 1**


This example has emergency syslog messages being sent to an IP location, debug (and all higher severity)
syslog messages to another IP address, and critical and higher syslog messages to local storage (either a
file or SGD).


771


SGD Printer Commands

```
! U1 setvar "device.syslog.configuration"
"emerg,128.168.0.1;debug,192.168.0.2;crit,local;"

```

**Example 2**


This example will only report emegency syslog messages to the local file, and ignore the duplicate location
request for critical and higher reports.

```
! U1 setvar "device.syslog.configuration" "emerg,local;crit,local;"

```

**Example 3**

```
This is an example of a syslog report stored at E:SYSLOG.TXT. Note that
device.syslog.save_local_file must be enabled.
Feb 17 14:28:17: [Power][Informational][0X14] Power On
Feb 17 14:28:19: [Print][Informational][0XF] PQ Job Completed
Feb 17 14:28:20: [Print][Informational][0XF] PQ Job Completed
Feb 17 14:28:20: [Weblink][Informational][0X1005] Weblink disabled
Feb 17 14:28:34: [Network][Notice][0X1C] Cold Start

```

**Example 4**


This is an example of a syslog report from a syslog server application monitoring an IP address.


772


SGD Printer Commands
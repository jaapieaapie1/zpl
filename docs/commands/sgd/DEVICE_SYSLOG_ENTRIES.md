# device.syslog.entries



This printer setting displays previously sent syslog messages. If there are no previously sent syslog
messages, an empty string is returned.


**Getvar**


To display previously sent syslog messages:

```
       ! U1 getvar "device.syslog.entries"

```

The format of each syslog message includes the printer feature, the severity level, the unique message
code, and the unique English message. This allows for more advanced systems administrators to filter
particular messages of interest. Syslog currently supports unique messages for most printer alerts,
WebLink, and some USB Host messages.


**Example**

This `getvar` example shows the value of the syslog file.

```
       ! U1 getvar "device.syslog.entries"

```

returns

```
       Feb 17 14:28:17: [Power][Informational][0X14] Power On
       Feb 17 14:28:19: [Print][Informational][0XF] PQ Job Completed
       Feb 17 14:28:20: [Print][Informational][0XF] PQ Job Completed
       Feb 17 14:28:20: [Weblink][Informational][0X1005] Weblink disabled
       Feb 17 14:28:34: [Network][Notice][0X1C] Cold Start

```

774


SGD Printer Commands
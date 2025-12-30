# device.syslog.log_max_file_size



This printer setting specifies the maximum size of the local syslog file.


**Setvar**


To set the maximum syslog file size to the specified value:

```
       ! U1 setvar "device.syslog.log_max_file_size" "value"

```

**Values**


A numerical value between 10000 and 400000


**Default**
```
          "10000"

```

**Getvar**


To return the maximum allowed size of the syslog file:

```
       ! U1 getvar "device.syslog.log_max_file_size"

```

**Example**

This `setvar` example shows the value set to `"200000"` .

```
       ! U1 setvar "device.syslog.log_max_file_size" "200000"

```

775


SGD Printer Commands
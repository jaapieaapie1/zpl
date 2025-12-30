# device.syslog.save_local_file



This command saves the contents of the local syslog to E:SYSLOG.TXT. The local destination must be
specified in `device.syslog.configuration` .


**Setvar**


To specify whether to save the contents of the local syslog file to E:SYSLOG.TXT:

```
       ! U1 setvar "device.syslog.save_local_file" "value"

```

**Values**

              - `"yes"` the local syslog is saved to E:SYSLOG.TXT

              - `"no"` the local syslog is not saved

**Default**
```
          "no"

```

**Getvar**


To display the setting for saving the local syslog file to E:SYSLOG.TXT:

```
       ! U1 getvar "device.syslog.save_local_file"

```

**Example**

This `setvar` example shows the value set to `"yes"` .

```
       ! U1 setvar "device.syslog.save_local_file" "yes"

```

776


SGD Printer Commands
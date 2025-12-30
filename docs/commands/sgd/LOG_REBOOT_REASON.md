# log.reboot.reason



Returns the reason for the last printer reboot, based on the `log.reboot.code` .


**Getvar**

To return the reason for the last printer reboot, based on the `log.reboot.code:`

```
       ! U1 getvar "log.reboot.reason"

```

**Result**


The reason for the last reboot.


**Values**

```
          "Other"
          "device.reset command"
          "Mirror reset – new files"
          "DTR off"
          "Low-battery timeout"
          "Low-battery shutdown"
          "power.shutdown command"
          "Idle timeout"
          "New OS reprogramming"
          "Unknown-1"
          "Unknown-2"
          "Off key"
          "No data"

```

839


SGD Printer Commands
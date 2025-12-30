# device.protected_mode_allowed



This command retrieves information on whether the protected mode feature is supported in the printer
operating system.


**Getvar**


To determine whether the protected mode is allowed:

```
       ! U1 getvar "device.protected_mode_allowed"

```

**Values**

              - `"yes"` protected mode is allowed.

              - `"no"` protected mode is not allowed.

**Default**
```
          "yes"

```

742


SGD Printer Commands
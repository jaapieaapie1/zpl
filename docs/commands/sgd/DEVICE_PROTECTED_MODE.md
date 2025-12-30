# device.protected_mode



This command retrieves information on the protected mode feature settings.

It returns `"off"` if protected mode is currently disabled or `"on"` if protected mode is currently enabled.
Protected mode is enabled if a non-empty protected mode password for the administrator user has been
set.


**Getvar**


To retrieve the status of protected mode:

```
       ! U1 getvar "device.protected_mode"

```

**Values**

              - `"off"` protected mode is off.

              - `"on"` protected mode is on.

**Default**
```
          "off"

```

741


SGD Printer Commands
# interface.network.settings_require_reset



Displays whether or not the network system has to be reset for a new configuration setting to take effect.


**Getvar**


To display whether or not the network system has to be reset for a new configuration setting to take effect:

```
       ! U1 getvar "interface.network.settings_require_reset"

```

**Values**

              - `"no"` means no settings have been changed that require a reset to take effect.

              - `"yes"` means one or more settings has been changed that requires a reset to take effect.


1156


SGD Network Commands
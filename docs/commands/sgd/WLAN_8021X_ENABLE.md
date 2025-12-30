# wlan.8021x.enable



Enables or disables the 802.1x security protocol, with the option to set it to WPA security protocol.


**Setvar**


To set the command:

```
       ! U1 setvar "wlan.8021x.enable" "value"

```

**Values**

              - `"off"` 802.1x security protocol is disabled.

              - `"on"` 802.1x security protocol is enabled.

              - `"wpa"` 802.1x security protocol is enabled and uses WPA.

**Default**
```
          "off"

```

**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "wlan.8021x.enable"

```

1369


SGD Network Commands
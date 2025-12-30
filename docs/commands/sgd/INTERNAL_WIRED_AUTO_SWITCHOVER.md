# internal_wired.auto_switchover



This command instructs the printer to switch from wireless to the internal wired print server when an
Ethernet cable is plugged into the printer and the printer detects an active data link.


**Setvar**


To configure switches between the wireless and wired interfaces:

```
       ! U1 setvar "internal_wired.auto_switchover" "value"

```

**Values**

              - `"on"` indicates switchover enabled

              - `"off"` indicates switchover disabled

**Default**
```
          "off"

```

**Getvar**


To retrieve the current automatic switchover value:

```
       ! U1 getvar "internal_wired.auto_switchover"

```

**Example**

This `setvar` example shows the value set to `"off"` .

```
       ! U1 setvar "internal_wired.auto_switchover" "off"

```

When the `setvar` value is set to `"off"`, the `getvar` result is `"off"` .


**IMPORTANT:** For this command to work, be sure:


          - you are using a ZM400/ZM600 or RZ400/RZ600 printer with both the internal 10/100 wired
print server and wireless option board installed

          - the value for this command is set to `"on"` (switchover enabled)

          - the printer is currently communicating to the network through a wireless connection


          - a Ethernet cable is plugged into the ZM400/ZM600 or RZ400/RZ600 printer and the printer
recognizes a data link connection


When the above conditions exist and an active Ethernet cable is plugged into an internal wired print server,
the printer will detect the wired data link and automatically switch to the wired interface. The printer will
automatically switch back to the wireless interface when the Ethernet cable is disconnected.


1166


SGD Network Commands
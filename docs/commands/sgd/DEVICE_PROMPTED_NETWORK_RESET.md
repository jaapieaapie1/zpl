# device.prompted_network_reset



Reinitializes the wireless radio card and the print server (wired or wireless) when the Wireless or Wireless
Plus print server is running. The command also causes any wireless radio card in the printer to re-associate
to the wireless network.


This command is equivalent to the ~WR - Reset Wireless on page 400.


**Setvar**


To set the device prompted reset:

```
       ! U1 setvar "device.prompted_network_reset" "value"

```

**Values**

              - `"yes"` causes the network to reset

              - `"no"` no changes to the network


738


SGD Printer Commands
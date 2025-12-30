# wlan.operating_mode



This printer setting refers to the network operating mode. Infrastructure mode means that the printer will
try to associate with an access point. Ad hoc mode means that the printer will try to associate with a device
other than an access point and join a standalone network.

To use `"ad hoc"` mode, configure the printer as follows:

          - Set the ESSID to the new network’s ESSID.


          - Turn off the DHCP and assign an IP Address to the printer.


          - Set the subnet mask on the printer to the new network’s subnet mask.

          - Change the operating mode on the printer to `"ad hoc"` .


**Setvar**


To set the network operating mode:

```
       ! U1 setvar "wlan.operating_mode" "value"

```

**Values**

              - `"adhoc"` means the printer will try to associate with a network device

`"infrastructure"` means the printer will try to associate with an access point


**Getvar**


To respond with the network-mode value:

```
       ! U1 getvar "wlan.operating_mode"

```

**Example**

This `setvar` example shows the value set to `"infrastructure"` .

```
       ! U1 setvar "wlan.operating_mode" "infrastructure"

```

When the `setvar` value is set to `"infrastructure"`, the `getvar` result is `"infrastructure"` .


1461


SGD Network Commands
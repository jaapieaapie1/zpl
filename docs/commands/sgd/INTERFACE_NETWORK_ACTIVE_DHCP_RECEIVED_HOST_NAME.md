# interface.network.active.dhcp_received_host_name



This command reports the Host Name as assigned by the DHCP Server. If one is not assigned, or DHCP is
not used, then the field will be blank.


**NOTE:** This command will only give a valid response once an IP address has been established.


**Getvar**


To report the host name:

```
       ! U1 getvar "interface.network.active.dhcp_received_host_name"

```

**Example**

```
       ! U1 getvar "interface.network.active.dhcp_received_host_name"

```

**Result**

```
          "Zebra Printer on shelf 2112"

```

1137


SGD Network Commands
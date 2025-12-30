# device.super_host_status



This command returns printer description information in XML format. The printer returns information on
format parameters, object directories, individual object data, and print status information.

This command is equivalent to the `^HZA` ZPL command.


**Getvar**


To return printer description information in XML format:

```
       ! U1 getvar "device.super_host_status"

```

**Result**


Information on format parameters, object directories, individual object data, and print status
information.


769


SGD Printer Commands
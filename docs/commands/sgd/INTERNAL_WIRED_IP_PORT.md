# internal_wired.ip.port



This printer setting refers to the internal wired print servers port number that the TCP print service is
listening on. Normal TCP communications from the host should be directed to this port.


**Setvar**


To instruct the printer to set the internal wired print servers TCP/UDP port number:

```
       ! U1 setvar "internal_wired.ip.port" "value"

```

**Values**


1 - 65535 (excluding any ports currently used by other services, such as 21, 23, 80, and 515).


**Default**
```
          "9100"

```

**Getvar**


To instruct the printer to respond with the internal wired printer servers TCP/UDP port number:

```
       ! U1 getvar "internal_wired.ip.port"

```

**Example**

This `setvar` example shows the value set to `"9100"` .

```
       ! U1 setvar "internal_wired.ip.port" "9100"

```

When the `setvar` value is set to `"9100"`, the `getvar` result is `"9100"` .


1193


SGD Network Commands
# internal_wired.ip.netmask



This setting refers to the internal wired print server’s subnet mask address. This value is ignored if the IP
protocol is not set to permanent.


**Setvar**


To instruct the printer to change the internal wired print servers subnet mask:

```
       ! U1 setvar "internal_wired.ip.netmask" "value"

```

**Values**


Any valid subnet mask.


**Default**
```
          "255.255.255.0"

```

**Getvar**


To instruct the printer to respond with internal wired print servers subnet mask:

```
       ! U1 getvar "internal_wired.ip.netmask"

```

**Example**

This `setvar` example shows the value set to `"255.255.255.0"` .

```
       ! U1 setvar "internal_wired.ip.netmask" "255.255.255.0"

```

When the `setvar` value is set to `"255.255.255.0"`, the `getvar` result is `"255.255.255.0"` .


1192


SGD Network Commands
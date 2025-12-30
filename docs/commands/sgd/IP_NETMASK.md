# ip.netmask



This setting refers to the subnet mask address. This value is ignored if the IP protocol is not set to
permanent.


**Setvar**


To instruct the printer to change the subnet mask:

```
       ! U1 setvar "ip.netmask" "value"

```

**Values**


Any valid subnet mask.


**Default**
```
          "255.255.255.0"

```

**Getvar**


To respond with the subnet mask value:

```
       ! U1 getvar "ip.netmask"

```

**Example**

This `setvar` example shows the value set to `"255.255.255.0"` .

```
       ! U1 setvar "ip.netmask" "255.255.255.0"

```

When the `setvar` value is set to `"255.255.255.0"`, the `getvar` result is `"255.255.255.0"` .


1290




SGD Network Commands
# ip.mirror.server



This command identifies the IP address of the mirroring server.


**Setvar**


To set the IP address of the mirroring server:

```
       ! U1 setvar "ip.mirror.server" "value"

```

**Values**


A valid IP address


**Default**
```
          "127.0.0.1"

```

**Getvar**


To retrieve the IP address of the mirroring server:

```
       ! U1 getvar "ip.mirror.server"

```

**Example**

This `setvar` example shows the value set to `"10.3.1.1"` .

```
       ! U1 setvar "ip.mirror.server" "10.3.1.1"

```

When the `setvar` value is set to `"10.3.1.1"`, the `getvar` result is `"10.3.1.1"` .


1285


SGD Network Commands
# ip.tcp.nagle_algorithm



Enables or disables the use of the Nagle algorithm on TCP connections.


**Setvar**


To enables or disable the use of the Nagle algorithm on TCP connections:

```
       ! U1 setvar "ip.tcp.nagle_algorithm" "value"

```

**Values**

`"enabled"` allows the use of the Nagle algorithm

`"disabled"` disables use of the Nagle algorithm

**Default**
```
          "enabled"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "ip.tcp.nagle_algorithm"

```

1325


SGD Network Commands
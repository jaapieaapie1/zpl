# internal_wired.ip.timeout.value



This network setting refers to the number of seconds before the connection times out for the internal wired
print server. For this to take effect, the print server must be reset.


**Setvar**


To instruct the printer to set the time of the internal wired print server, in seconds, before the connection
times out:

```
       ! U1 setvar "internal_wired.ip.timeout.value" "value"

```

**Values**

`"1"` through `"3600"`

**Default**
```
          "300"

```

**Getvar**


To instruct the printer to respond with the time of the internal wired print server, in seconds, before the
connection times out:

```
       ! U1 getvar "internal_wired.ip.timeout.value"

```

**Example**

This `setvar` example shows the value set to `"300"` .

```
       ! U1 setvar "internal_wired.ip.timeout.value" "300"

```

When the `setvar` value is set to `"300"`, the `getvar` result is `"300"` .


1198


SGD Network Commands
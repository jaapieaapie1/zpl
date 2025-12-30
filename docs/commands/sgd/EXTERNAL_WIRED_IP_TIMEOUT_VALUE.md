# external_wired.ip.timeout.value



This network setting refers to the number of seconds before the connection times out for the external
wired print server.


**Setvar**


To instructs the printer to set the time of the external wired print server, in seconds, before the connection
times out:

```
       ! U1 setvar "external_wired.ip.timeout.value" "value"

```

**Values**

`"1"` through `"3600"`

**Default**
```
          "300"

```

**Getvar**


To instruct the printer to respond with the time of the external wired print server, in seconds, before the
connection times out:

```
       ! U1 getvar "external_wired.ip.timeout.value"

```

On SEH print server models PS102-Z or the PS105-Z, only the getvar command is supported.


**Example**

This `setvar` example shows the value set to `"300"` .

```
       ! U1 setvar "external_wired.ip.timeout.value" "300"

```

When the `setvar` value is set to `"300"`, the `getvar` result is `"300"` .


1129


SGD Network Commands
# ip.http.port



This command sets the port number at which the printer web pages will be served.


**Setvar**


To change the port setting for the printer web pages:

```
       ! U1 setvar "ip.http.port" "value"

```

**Values**

`"0"` to `"65535"`

**Default**
```
          "80"

```

**Getvar**


To respond with the current port setting for the printer web pages:

```
       ! U1 getvar "ip.http.port"

```

**Example**

This `setvar` example shows the value set to `"8080"` .

```
       ! U1 setvar "ip.http.enable" "8080"

```

1261


SGD Network Commands
# ip.ntp.servers



Sets the list of NTP (Network Time Protocol) servers which the printer will use to set the time.


**Setvar**


To set the list of NTP (Network Time Protocol) servers which the printer will use to set the time:

```
       ! U1 setvar "ip.ntp.servers" "value"

```

**Values**


A comma delimited string of server name(s) or ip address(es), with a length of 0-1024 characters.


**Default**
```
          ""

```

**Example**

```
       ! U1 setvar "ip.ntp.servers" "0.us.pool.ntp.org,10.3.17.124"

```

**Getvar**


To retrieve the current setting value:

```
       ! U1 getvar "ip.ntp.servers"

```

**Result**


A comma delimited string of server name(s) or ip address(es).


1293


SGD Network Commands
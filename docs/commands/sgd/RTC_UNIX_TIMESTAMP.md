# rtc.unix_timestamp



This command sets or gets the printer time based on the Unix Epoch (UTC) number of seconds since
January 1, 1970.


**Setvar**


To set the command:

```
       ! U1 setvar "rtc.unix_timestamp" "123123"

```

**Values**

`"0"` to `"0xFFFFFFFF"`


**Getvar**


To get the current printer time in seconds since 1970:

```
       ! U1 getvar "rtc.unix_timestamp"

```

978


SGD Printer Commands
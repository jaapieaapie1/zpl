# rtc.timezone



This command specifies the POSIX-compliant time zone string.


This string includes the following:


          - the time zone character specifier


          - the offset from UTC


          - daylight savings time adjustment


          - when to go on and off of daylight savings time (if it pertains to the timezone).


**Setvar**


To set the POSIX-compliant time zone string:

```
       ! U1 setvar "rtc.timezone" "value"

```

**Values**


[www.iana.org/time-zones](http://www.iana.org/time-zones) This site is updated periodically to reflect changes made by political
bodies to time zone boundaries, UTC offsets, and daylight-saving rules.


**Getvar**


To return the current setting value:

```
       ! U1 getvar "rtc.timezone"

```

**Example**

If you live in New York in the United States, in the Eastern time zone, your `setvar` string may look like:

```
       ! U1 setvar "rtc.timezone" "EST5EDT4,M3.2.0/02:00:00,M11.1.0/02:00:00"

```

The `"value"` string can be translated as follows: `EST5` (Eastern Standard Time; 5 hours off UTC), `EDT4`
(Eastern Daylight Time; 4 hours off UTC), running from 2AM ( `/02` ; fully qualified: `/02:00:00` ) from the
second Sunday in March ( `M3.2.0/02` ) through 2AM ( `/02` ; fully qualified: `/02:00:00` ) on the first Sunday
in November ( `M11.1.0/02` ).

`M` indicates the Month follows, followed by the two-digit month, the week ( `1` is the first week in which the
specified weekday occurs, and `5` indicates the last week of the month with that weekday) and the weekday
( `0` is Sunday). The time starts with a slash, and unspecified trailing fields default to zero.

Other examples for locations in the United States:


          - US Central:

```
         ! U1 setvar "rtc.timezone" "CST6CDT5,M3.2.0/02,M11.1.0/02"

```

          - US Mountain:

```
         ! U1 setvar "rtc.timezone" "MST7MDT6,M3.2.0/02,M11.1.0/02"

```

976


SGD Printer Commands


- US Pacific:

```
 ! U1 setvar "rtc.timezone" "PST8PDT7,M3.2.0/02,M11.1.0/02"

```

- US Alaska:

```
 ! U1 setvar "rtc.timezone" "AST9ADT8,M3.2.0/02,M11.1.0/02"

```

- US Hawaii:

```
 ! U1 setvar "rtc.timezone" "HST10"

```

977


SGD Printer Commands
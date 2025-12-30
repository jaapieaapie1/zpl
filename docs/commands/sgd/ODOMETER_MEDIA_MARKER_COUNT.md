# odometer.media_marker_count



This command refers to the non-resettable media marker count. The media marker counter keeps track
of how many labels have passed through the printer by counting the bar sense marks on the back of the
media or the gap in gap media. Labels are counted whether or not they have been printed.


**Setvar**


To set the non-resettable media marker count:

```
       ! U1 setvar "odometer.media_marker_count" "value"

```

**Values**

`"0"` to `"4294967295"`

**Default**
```
          "0"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "odometer.media_marker_count"

```

**Result**

```
          "105"

```

921


SGD Printer Commands
# capture.channel1.max_length



This command sets a length indicating when to copy captured data to the data SGD if the delimiter has not
been seen yet.


If the delimiter and the max_length are reached at the same time, the delimiter will not be part of the
captured data. Of only part of the delimiter has been received, then the part of the delimiter we have
received, will be part of the capture data.


When the max_length is changed, any data currently in the buffer will be thrown away, and the new value
of max_length will be used.

The Capture Port shall be defaulted to 1000 bytes by any mechanism (including `^JUF`, `^JUN`, `^JUA`, and
`device.restore_defaults` ).


**Setvar**


To instruct the printer to set a default data capture length:

```
       ! U1 setvar "capture.channel1.max_length" "value"

```

**Values**

`"1"` to `"3000"`

**Default**
```
          "1000"

```

**Getvar**


To retrieve the default data capture length:

```
       ! U1 getvar "capture.channel1.max_length"

```

646


SGD Printer Commands
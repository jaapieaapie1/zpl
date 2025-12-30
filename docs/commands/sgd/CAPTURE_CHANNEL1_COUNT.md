# capture.channel1.count



This command indicates the number of times that `capture.channel1.delimiter` was seen
on the port specified in `capture.channel1.port` . Additionally, it indicates how many times
`capture.channel1.data.raw` has been updated with user data as well as the number of times we
reached the `capture.channel1.max_length` .

This will be shown in the `HZA` response under the capture data section.


**Getvar**

To return the number of times that `capture.channel1.delimiter` was seen on the port specified in
`capture.channel1.port` :

```
       ! U1 getvar "capture.count"

```

642


SGD Printer Commands
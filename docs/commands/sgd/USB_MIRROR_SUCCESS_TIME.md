# usb.mirror.success_time



This command returns the date and time of the last successful USB mirroring operation. The date and time
reported are from the printer’s clock.


**Getvar**


To return the date and time of the last successful USB mirroring operation:

```
       ! U1 getvar "usb.mirror.success_time.fm"

```

**Result**
```
          02 18 2015 11:10:09
```

In mm dd yyyy hh:mm:ss format.


1045


SGD Printer Commands
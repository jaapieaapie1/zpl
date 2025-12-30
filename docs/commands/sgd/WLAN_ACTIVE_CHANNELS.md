# wlan.active_channels



This command returns list of current active channels defined by `wlan.allowed_band` and
`wlan.user_channel_list` .


**Getvar**


To return the list of current active channels:

```
       ! U1 getvar "wlan.active_channels"

```

**Result**


One or more of the following channels:
```
          1,2,3,4,5,6,7,8,9,10,11,36,40,44,48,52,56,60,64,100,104,108,
          112,116,132,136,140,149,153,157,161,165,all

```

1380




SGD Network Commands
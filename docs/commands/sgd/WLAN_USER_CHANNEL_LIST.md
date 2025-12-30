# wlan.user_channel_list



This command sets the list of available channels.


**Setvar**


To set the list of available channels:

```
       ! U1 setvar "wlan.user_channel_list" "value"

```

**Values**


One or more of the following:
```
          "1,2,3,4,5,6,7,8,9,10,11,36,40,44,48,52,56,60,64,100,104,108,112,116,132,136,140
          "all"
```

**Default**
```
          "all"

```

**Getvar**


To retrieve the currently set list of available channels:

```
       ! U1 getvar wlan.user_channel_list"

```

**Example**


This example sets the available channel list to channels 1-9.

```
       ! U1 setvar "wlan.user_channel_list" "1,2,3,4,5,6,7,8,9"

```

1494


SGD Network Commands
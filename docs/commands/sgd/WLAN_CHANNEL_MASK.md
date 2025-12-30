# wlan.channel_mask



This command specifies the wireless channel masks for enabling and disabling various channels. This
controls which b/g radio channels can be used by the radio for network connections.


The value for this command is a bit field where a 0 disables a channel and a 1 enables the channel. Starting
from the right, bit 0 is for channel 1, bit 1 for channel 2, etc. This can be used to limit the channels scanned
for networks, which may slightly improve connection and roaming speed. It also used to ensure compliance
with the regulatory domains of your location.

|Commonly Used Channel Mask Settings|Col2|Col3|
|---|---|---|
|**Region**|**Channels**|**Channel Mask**|
|United States, Canada, Latin America|1 - 11|0x7FF|
|Europe, Middle East, Africa, other|1 - 13|0x1FFF|
|Japan|1 - 14|0x3FFF|



**NOTE:** This command is not supported by all radios. Ensure the channel masks are set in
accordance with the regulatory domains of your country.


**Setvar**


To instruct the printer to set the wireless channel mask value:

```
       ! U1 setvar "wlan.channel_mask" "value"

```

**Values**

`"0x0000"` to `"0xFFFF"` (4 hexadecimal digits preceded by `"0x"` )

**Default**
```
          "0x7FF"

```

**Getvar**


To instruct the printer to set the wireless channel mask value:

```
       ! U1 getvar "wlan.channel_mask"

```

**Example 1**

This `setvar` example shows the value set to `"0x7FF" for common North American channels` .

```
       ! U1 setvar "wlan.channel_mask" "0x7FF"

```

**Example 2**


This setvar example sets the channel mask to use only channels 1,6,11.

```
       ! U1 setvar "wlan.channel_mask" "0x421"

```

Only channels 1, 6, and 11 will be used by the radio.


1393


SGD Network Commands
# wlan.wep.key1



Use this command to set the first indexed WEP encryption key. The WEP encryption key is a hexadecimal
or string value. This key should match the wireless network WEP encryption key 1.


**NOTE:** This command is not supported for printers running Link-OS 6 or later versions.


**Setvar**


To set the encryption key:

```
       ! U1 setvar "wlan.wep.key1" "value"

```

**Values**


              - 10 hexadecimal characters for 40-bit encryption


              - 26 hexadecimal characters for 128-bit encryption


**Default**


All zeros


**Getvar**


To instruct the printer to respond with the encryption key:

```
       ! U1 getvar "wlan.wep.key1"

```

For protection a single `"*"` prints.


**Example**

This `setvar` example shows the value set to `"A1B2C3D4F5"` .

```
       ! U1 setvar "wlan.wep.key1" "A1B2C3D4F5"

```

When the `setvar` value is set to `"A1B2C3D4F5"`, the `getvar` result is `"*"` .


1498


SGD Network Commands
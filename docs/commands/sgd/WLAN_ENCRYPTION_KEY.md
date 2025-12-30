# wlan.encryption_key



These parameters refer to the first, second, third, and fourth indexed WEP encryption keys. The WEP
encryption keys are hexadecimal strings that are either 10 or 26 characters long depending on the
encryption method (40-bit or 128 bit). The keys should match the wireless network WEP encryption keys.


**Setvar**


To set the encryption key value:

```
       ! U1 setvar "wlan.encryption_key[1|2|3|4]" "value"

```

**Values**


10 hexadecimal characters for 40-bit encryption and 26 hexadecimal characters for 128-bit
encryption.


**Default**


All zeroes (10 or 26, depending on encryption setting)


**Getvar**


To instruct the printer respond with the encryption key value:

```
       ! U1 getvar "wlan.encryption_key1"

```

**Example 1**

In these examples, the `getvar` results assume that the printer is using 40-bit encryption with the default
settings.

```
       ! U1 getvar "wlan.encryption_key1"

       ! U1 getvar "wlan.encryption_key2"

       ! U1 getvar "wlan.encryption_key3"

       ! U1 getvar "wlan.encryption_key4"

```

Results for each key: `"0000000000"`


**Example 2**

In these examples, the `setvar` command instructs the printer to set the encryption key value. This
example assumes that the printer is using 40-bit encryption.

```
       ! U1 setvar "wlan.encryption_key1" "A1B2C3D4F5"

```

1398


SGD Network Commands

```
! U1 setvar "wlan.encryption_key2" "G1H2J3K4L5"

! U1 setvar "wlan.encryption_key3" "M1N2P3Q4R5"

! U1 setvar "wlan.encryption_key4" "S1T2V3W4X5"

```

1399


SGD Network Commands
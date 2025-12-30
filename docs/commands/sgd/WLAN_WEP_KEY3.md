# wlan.wep.key3



Use this command to set the third indexed WEP encryption key. The WEP encryption key is a hexadecimal
string value. This key should match the wireless network WEP encryption key 3.


**NOTE:** This command is not supported for printers running Link-OS 6 or later versions.


**Setvar**


To set the encryption key:

```
       ! U1 setvar "wlan.wep.key3" "value"

```

**Values**


              - 10 hexadecimal characters for 40-bit encryption


              - 26 hexadecimal characters for 128-bit encryption


**Default**


All zeros


**Getvar**


To instruct the printer to respond with the encryption key:

```
       ! U1 getvar "wlan.wep.key3"

```

For protection a single `"*"` prints.


**Example**

This `setvar` example shows the value set to `"A1B2C3D4F5"` .

```
       ! U1 setvar "wlan.wep.key3" "A1B2C3D4F5"

```

When the `setvar` value is set to `"A1B2C3D4F5"`, the `getvar` result is `"*"` .


1500




SGD Network Commands

## **wlan.wep.key4**


Use this command to set the fourth indexed WEP encryption key. The WEP encryption key is a
hexadecimal string value. This key should match the wireless network WEP encryption key 4.


**NOTE:** This command is not supported for printers running Link OS 6 or later versions.


**Setvar**


To set the encryption key:

```
       ! U1 setvar "wlan.wep.key4" "value"

```

**Values**


10 hexadecimal characters for 40-bit encryption


26 hexadecimal characters for 128-bit encryption


**Default**


All zeros


**Getvar**


To respond with the encryption key:

```
       ! U1 getvar "wlan.wep.key4"

```

For protection a single `"*"` prints.


**Example**

This `setvar` example shows the value set to `"A1B2C3D4F5"` .

```
       ! U1 setvar "wlan.wep.key4" "A1B2C3D4F5"

```

When the `setvar` value is set to `"A1B2C3D4F5"`, the `getvar` result is `"*"` .


1501


SGD Network Commands

## **wlan.wpa.psk**


This printer setting specifies the pre-shared key (PSK) value to use when the WPA authentication is set to
PSK.


**Setvar**


To set the pre-shared key:

```
       ! U1 setvar "wlan.wpa.psk" "value"

```

**Values**


64 hexadecimal digits


**Default**


64 zeros (00000000...)


**Getvar**


To return the pre-shared key value:

```
       ! U1 getvar "wlan.wpa.psk"

```

For protection a single `"*"` prints.


**Example**

This `setvar` example shows the value set to `"00000000..."` .

```
       ! U1 setvar "wlan.wpa.psk" "00000000..."

```

When the `setvar` value is set to `"00000000..."`, the `getvar` result is `"*"` .


1502


SGD Network Commands

## **wlan.wep.key_format**


This printer setting specifies the format for the WEP key.


This command is disabled for Link OS 6 printers and later versions.


**NOTE:** This printer setting should proceed any of the `wep.key` settings if you select a nondefault value.


**Setvar**


To set the WEP key format:

```
       ! U1 setvar "wlan.wep.key_format" "value"

```

**Values**

              - `"ascii"` the WEP key is set by ASCII string

              - `"hex"` the WEP key is a Hex string

**Default**
```
          "hex"

```

**Getvar**


To respond with the WEP key format:

```
       ! U1 getvar "wep.key_format"

```

**Example**

This `setvar` example shows the value set to `"ascii"` .

```
       ! U1 setvar "wlan.wep.key_format" "ascii"

```

When the `setvar` value is set to `"ascii"`, the `getvar` result is `"ascii"` .


1503


SGD Network Commands

## **wlan.wpa.groupkey_ciphersuite**


This command returns the encryption method currently used for unicast packets.


**Getvar**


To return the current encryption method value:

```
       ! U1 getvar "wlan.wpa.groupkey_ciphersuite"

```

**Result**

`"NONE"` No encryption being used.

`"WEP40"` WEP40 encryption being used.

`"TKIP"` TKIP encryption being used.

`"AES"` AES being used.

`"WEP104"` WEP104 encryption being used.

`"WPA2"` WEP104 encryption being used.

`""` Reported if printer is not yet associated with the wireless LAN.


1504


SGD Network Commands

## **wlan.wpa.pairwise_ciphersuite**


This command returns the encryption method currently used for unicast packets.


**Getvar**


To return the value of the current encryption methods:

```
       ! U1 getvar "wlan.wpa.pairwise_ciphersuite"

```

**Result**

`"NONE"` No encryption being used.

`"WEP40"` WEP40 encryption being used.

`"TKIP"` TKIP encryption being used

`"AES"` AES being used.

`"WEP104"` WEP104 encryption being used.

`"WPA2"` WPA2 encryption being used.

`""` Reported if printer is not yet associated with the wireless LAN.


1505


SGD Network Commands

## **wlan.wpa.timecheck**


Allows the user to disable the certificate timestamp check that is performed during a WPA TLS handshake.


**Setvar**


To enable or disable the certificate timestamp check that is performed during a WPA TLS handshake:

```
       ! U1 setvar "wlan.wpa.timecheck" "value"

```

**Values**

              - `"yes"` means the timecheck during the handshake will be performed

              - `"no"` means the timecheck during the handshake will not be performed

**Default**

" `yes"`


**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.wpa.timecheck"

```

1506


SGD Network Commands

## **wlan.wpa.wpa_version**


Returns the currently active WPA version.


**Getvar**


To return the currently active WPA version:

```
       ! U1 getvar "wlan.wpa.wpa_version"

```

**Values**

`"WPA"` indicates WPA is being used.

`"WPA2"` indicates WPA2 is being used.

`"WPA3"` indicates that WPA3 is being used.

`""` reported if the WLAN is not connected or WPA is not enabled.


1507
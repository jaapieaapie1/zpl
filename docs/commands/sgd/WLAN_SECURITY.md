# wlan.security



This printer setting allows you to specify both the wireless encryption type and authentication type in one
command.


**NOTE:** The supporting parameters that are required vary based on the security type that you
select. See Supporting SGDs for Different Security Types on page 3 for instructions for each
security type.


When using certificate files, Zebra printers support:


          - using Privacy Enhanced Mail (PEM) formatted certificate files.


          - using the client certificate and private key as two files, each downloaded separately.


          - using exportable PAC files for EAP-FAST.


These certificate files can only be sent using ZPL, not SGD. The ZPL command to use when sending these
certificate files is the `~DY` command.

Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


**NOTE:** When using certificate files, the time on the printer must be set correctly for the
WebSocket connection to succeed, as the time is used in the certificate validation.


**Setvar**


To set the wireless security value:

```
       ! U1 setvar "wlan.security" "value"

```

1478


SGD Network Commands


**Values**

    - `"1"` No wireless security or `"none"`

    - `"2"` WEP 40-bit or `"wep 40-bit"`

    - `"3"` WEP 128-bit or `"wep 128-bit"`

    - `"4"` EAP-TLS or `"eap-tls"`

    - `"5"` EAP-TTLS or `"eap-ttls"`

    - `"6"` EAP-FAST or `"eap-fast"`

    - `"7"` PEAP or `"peap"`

    - `"8"` LEAP or `"leap"`

    - `"9"` WPA PSK or `"wpa psk"` (Key rotation for WPA2 PSK is supported in firmware versions
V53.15.8Z, V60.15.8Z, and later.)

    - `"10"` WPA EAP-TLS or `"wpa eap-tls"`

    - `"11"` WPA EAP-TTLS or `"wpa eap-ttls"`

    - `"12"` WPA EAP-FAST or `"wpa eap-fast"`

    - `"13"` WPA PEAP or `"wpa peap"`

    - `"14"` WPA LEAP or `"wpa leap"`

    - `"15"` Kerberos or `"kerberos"`

    - `"17"` WPA SAE or `"wpa sae"`

**Default**
```
   "1"

```

**Getvar**


To return the name and not the type:

```
! U1 getvar "wlan.security" "value"

```

If an invalid security mode is entered the printer returns `"Invalid Mode"` .


**Examples**

This `setvar` example shows the value set to `"1"` .

```
! U1 setvar "wlan.security" "1"

```

When the `setvar` value is set to `"1"`, the `getvar` result is `"none"` .


1479


SGD Network Commands

## **Supporting SGDs for Different Security Types**


The supporting SGD commands required for `wlan.security` vary based on the security type that
you select. You must send the additional commands for your printer to be able to work on your wireless
network. Follow the example and format for your specific security type in this section, substituting your own
wireless network data.


**Security Type 1: No Wireless Security Active**


Additional parameters that need to be set: none


**Example**


This example turns off all wireless securities controlled under this command, but it does not reset the
printer’s wireless settings to their defaults.

```
     ! U1 setvar "wlan.security" "1"

```

**Security Type 2: WEP 40-Bit**


Additional parameters that need to be set and the SGD commands to use:

        - WEP encryption index (see `wlan.wep.index` )

        - WEP authentication type (see `wlan.wep.auth_type` )

        - WEP key type (see `wlan.wep.key_format` )

        - the actual values of any WEP encryption keys to be used (see `wlan.wep.key1`, `wlan.wep.key2`,
`wlan.wep.key3`, or `wlan.wep.key4` )


**Example**


This example configures the printer for WEP 40-bit encryption using index key 1, open authentication, and a
hexadecimal WEP key with a value of `“A1B2C3D4F5”` .

```
     ! U1 setvar "wlan.security" "2"

     ! U1 setvar "wlan.wep.index" "1"

     ! U1 setvar "wlan.wep.auth_type" "open"

     ! U1 setvar "wlan.wep.key_format" "hex"

     ! U1 setvar "wlan.wep.key1" "A1B2C3D4F5"

```

**Security Type 3: WEP 128-Bit**


Additional parameters that need to be set and the SGD commands to use:

        - WEP encryption index (see `wlan.wep.index` )


1480




SGD Network Commands


- WEP authentication type (see `wlan.wep.auth_type` )

- WEP key type (see `wlan.wep.key_format` )

- the actual values of any WEP encryption keys to be used (see `wlan.wep.key1`, `wlan.wep.key2`,
`wlan.wep.key3`, or `wlan.wep.key4` )


**Example**


This example configures the printer for WEP 128-bit encryption using index key 2, open authentication, and
four hexadecimal WEP keys.

```
! U1 setvar "wlan.security" "3"

! U1 setvar "wlan.wep.index" "2"

! U1 setvar "wlan.wep.auth_type" "open"

! U1 setvar "wlan.wep.key_format" "hex"

! U1 setvar "wlan.wep.key1" "001122334455667788"

! U1 setvar "wlan.wep.key2" "112233445566778899"

! U1 setvar "wlan.wep.key3" "223344556677889900"

! U1 setvar "wlan.wep.key4" "334455667788990011"

```

**Security Type 4: EAP-TLS**


Additional parameters that need to be set and the SGD commands to use:

- optional private key password (see `wlan.private_key_password` )


**Example**


This example configures the printer for EAP-TLS authentication with an optional private key password with
a value of “ `private` .”

```
! U1 setvar "wlan.security" "4"

! U1 setvar "wlan.private_key_password" "private"

```

1481


SGD Network Commands


**Security Type 5: EAP-TTLS**


Additional parameters that need to be set and the SGD commands to use:

- user ID (see `wlan.username` )

- password (see `wlan.password` )


**Example**

With a value of `"user"` and a password with a value of `"password"` .

```
! U1 setvar "wlan.security" "5"

! U1 setvar "wlan.username" "user"

! U1 setvar "wlan.password" "password"

```

**Security Type 6: EAP-FAST**


Additional parameters that need to be set and the SGD commands to use:

- user ID (see `wlan.username` )

- password (see `wlan.password` )

- optional private key password (see `wlan.private_key_password` )


**Example**

This example configures the printer for EAP-FAST authentication, including a user ID of `"user"`, a
password of `"password"`, and an optional private key of `"private"` .

```
! U1 setvar "wlan.security" "6"

! U1 setvar "wlan.username" "user"

! U1 setvar "wlan.password" "password"

! U1 setvar "wlan.private_key_password" "private"

```

**Security Type 7: PEAP**


Additional parameters that need to be set and the SGD commands to use:

- user ID (see `wlan.username` )

- password (see `wlan.password` )


1482


SGD Network Commands


**Example**

This example configures the printer for PEAP authentication, including a user ID with a value of `"user"`
and a password with a value of `"password"` .

```
! U1 setvar "wlan.security" "7"

! U1 setvar "wlan.username" "user"

! U1 setvar "wlan.password" "password"

```

**Security Type 8: LEAP**


Additional parameters that need to be set and the SGD commands to use:

- user ID (see `wlan.username` )

- password (see `wlan.password` )


**Example**

This example configures the printer for LEAP authentication, including a user ID with a value of `"user"`
and a password with a value of `"password"` .

```
! U1 setvar "wlan.security" "8"

! U1 setvar "wlan.username" "user"

! U1 setvar "wlan.password" "password"

```

**Security Type 9: WPA PSK**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.
Key rotation for WPA2 PSK is supported in firmware version 60.15.8Z and later and in firmware
version 53.15.8Z and later.


Additional parameters that need to be set and the SGD commands to use:

- Pre-Shared Key (PSK) value (see `wlan.wpa.psk` )


**Example**


This example configures the printer for WPA PSK authentication with a PSK value of all zeroes (64
hexadecimal digits).

```
! U1 setvar "wlan.security" "9"

! U1 setvar "wlan.wpa.psk" "00000000..."

```

1483


SGD Network Commands


**Security Type 10: WPA EAP-TLS**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


Additional parameters that need to be set and the SGD commands to use:

- optional private key password (see `wlan.private_key_password` )


**Example**


This example configures the printer for WPA EAP-TLS authentication with an optional private key password
with a value of `"private"` .

```
! U1 setvar "wlan.security" "10"

! U1 setvar "wlan.private_key_password" "private"

```

**Security Type 11: WPA EAP-TTLS**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


Additional parameters that need to be set and the SGD commands to use:

- user ID (see `wlan.username` )

- password (see `wlan.password` )


**Example**


This example configures the printer for WPA EAP-TTLS authentication, including a user ID with a value of
`"user"` ” and a password with a value of `"password"` .

```
! U1 setvar "wlan.security" "11"

! U1 setvar "wlan.username" "user"

! U1 setvar "wlan.password" "password"

```

**Security Type 12: WPA EAP-FAST**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


Additional parameters that need to be set and the SGD commands to use:

- user ID (see `wlan.username` )

- password (see `wlan.password` )

- optional private key password (see `wlan.private_key_password` )


1484


SGD Network Commands


**Example**

This example configures the printer for WPA EAP-FAST authentication, including a user ID of `"user"`, a
password of `"password"`, and an optional private key of `"private"` .

```
! U1 setvar "wlan.security" "12"

! U1 setvar "wlan.username" "user"

! U1 setvar "wlan.password" "password"

! U1 setvar "wlan.private_key_password" "private"

```

**Security Type 13: WPA PEAP**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


Additional parameters that need to be set and the SGD commands to use:

- user ID (see `wlan.username` )

- password (see `wlan.password` )


**Example**


This example configures the printer for WPA PEAP authentication, including a user ID with a value of
`"user"` and a password with a value of `"password"` .

```
! U1 setvar "wlan.security" "13"

! U1 setvar "wlan.username" "user"

! U1 setvar "wlan.password" "password"

```

**Security Type 14: WPA LEAP**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


Additional parameters that need to be set and the SGD commands to use:

- user ID (see `wlan.username` )

- password (see `wlan.password` )


**Example**


This example configures the printer for WPA LEAP authentication, including a user ID with a value of
`"user"` and a password with a value of `"password"` .


1485


SGD Network Commands

```
! U1 setvar "wlan.security" "14"

! U1 setvar "wlan.username" "user"

! U1 setvar "wlan.password" "password"

```

**Security Type 15: Kerberos**


Additional parameters that need to be set and the SGD commands to use:

- Kerberos user ID (see `wlan.kerberos.username` )

- Kerberos password (see `wlan.kerberos.password` )

- realm (see `wlan.kerberos.realm` )

- Key Distribution Center (KDC) (see `wlan.kerberos.kdc` )


**Example**


This example configures the printer for Kerberos encryption, including a Kerberos user ID with a value
of `"user"`, a Kerberos password with a value of `"password"`, a realm of `"zebra"`, and a KDC of
`"krbtgt"` .

```
! U1 setvar "wlan.security" "15"

! U1 setvar "wlan.kerberos.username" "user"

! U1 setvar "wlan.kerberos.password" "password"

! U1 setvar "wlan.kerberos.realm" "zebra"

! U1 setvar "wlan.kerberos.kdc" "krbtgt"

```

1486


SGD Network Commands
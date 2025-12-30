# ^WX




ZPL Network Commands


Use this command to configure the wireless security settings for your printer. Values entered for this
command must match what is configured on your WLAN and must be supported by the wireless radio card
that you are using.


**Configure Wireless Securities**

The `^WX` command replaces individual ZPL commands for different security types.


**NOTE:**


When using certificate files, your printer supports:


- Using Privacy Enhanced Mail (PEM) formatted certificate files.


- Using the client certificate and private key as two files, each downloaded separately.


- Using exportable PAC files for EAP-FAST.


- The supporting parameters that are required vary based on the security type that you select.
See **Supporting Parameters for Different Security Types on page 429** for instructions for
each security type.


The values 2, 3 for the security type (a) parameter, b, c, d, e, f, g and h parameters are ignored for printer
running Link-OS 6.0 or later versions.


**NOTE:** **important:** When using certificate files, the time on the printer must be set correctly for
the websocket connection to succeed, as the time is used in the certificate validation.

**Format:** `^WXa,[zero or more supporting parameters]`


403


ZPL Network Commands







|Parameters|Details|
|---|---|
|a = security type|Enter the two-digit code for the security type that your WLAN uses. For<br>which supporting parameters (b through n) to use with the different security<br>types, see**Supporting Parameters for Differnt Security Types on page**<br>**429**.<br>**NOTE:** Configuring the printer for WPA also allows the printer to<br>be used in WPA2 environments.<br>**Values:**`01` to`15`<br>`01` - No wireless security is active<br>`02` = WEP 40-bit<br>`03` = WEP 128-bit<br>`04` = EAP-TLS<br>`05` = EAP-TTLS<br>`06` = EAP-FAST<br>`07` = PEAP<br>`08` = LEAP<br>`09` = WPA PSK (R6x15.x, R53.15.x, ZSPx, and later.)<br>`10` = WPA EAP-TLS<br>`11` = WPA EAP-TTLS<br>`12` = WPA EAP-FAST<br>`13` = WPA PEAP<br>`14` = WPA LEAP<br>`15` = Kerberos<br>**Default:**`01`|
|b = WEP encryption index*|Specifies which encryption key to use for WEP encryption. A value must be<br>specified if using WEP 40-bit or WEP 128-bit.<br>**Values:**1, 2, 3, 4<br>**Default:**1|
|c= WEP authentication<br>type*|Enables the WEP key authentication type. A value must be specified if using<br>WEP 40-bit or WEP 128-bit.<br>**Values:**`O` or`S`<br>`O` = open system<br>`S` = shared key<br>**Default:**`O`|
|d = WEP key type*|Specifies the format of the WEP key. A value must be specified if using<br>WEP40-bit or WEP 128-bit.<br>**Values:**`H` or`S`<br>`H` = hex key storage<br>`S` = string key storage<br>**Default:**`S`|


404


ZPL Network Commands







|Parameters|Details|
|---|---|
|e,f,g,h = WEP encryption<br>keys 1 through 4*|Specifies the actual values of any WEP encryption keys to be used. A value<br>must be specified for at least one WEP encryption key if you specify 40-bit<br>or 128-bit WEP encryption for the security type.<br>**NOTE:** **important:** Be careful to include the exact number of<br>commas required for this command when setting encryption keys<br>(parameters`e` through`h`). A missing or extra comma will cause the<br>keys to be stored in the wrong slots and can prevent the printer<br>from joining the wireless network.<br>The encryption mode affects what can be entered for the encryption keys:<br>•<br>For 40-bit, encryption keys can be set to any 5 hex pairs or any 10<br>alphanumeric characters.<br>•<br>For 128-bit, encryption keys can be set to any 13 hex pairs or any 26<br>alphanumeric characters.<br>**NOTE:** When using hex storage, do not add a leading 0x on the<br>WEP key.<br>**Values:**The actual value for the encryption key<br>**Default:**None|
|•<br>i = user ID*|Specifies a user ID for security types that require one. A value must be<br>specified if using the following security types:<br>•<br>EAP-TTLS<br>•<br>LEAP<br>•<br>WPA LEAP<br>•<br>PEAP<br>•<br>WPA PEAP<br>•<br>WPA EAP-TTLS<br>•<br>Kerberos<br>**Values:**The actual value for the user ID.<br>**Default:**user|


405


ZPL Network Commands













|Parameters|Details|
|---|---|
|•<br>j = password*|Specifies a password for security types that require one. A value must be<br>specified if using the following security types:<br>•<br>EAP-TTLS<br>•<br>LEAP<br>•<br>WPA LEAP<br>•<br>PEAP<br>•<br>WPA PEAP<br>•<br>WPA EAP-TTLS<br>•<br>Kerberos<br>**Values:**The actual value for the password.<br>**Default:**password|
|•<br>k = optional private<br>key password*|Specifies an optional private key password for security types that require<br>one. A value must be specified if using the following security types:<br>•<br>EAP-TLS<br>•<br>EAP-FAST<br>•<br>WPA EAP-TLS<br>•<br>WPA EAP-FAST<br>**Values:**The actual value for the optional private key.<br>**Default:**None|
|•<br>l = realm*|Specifies the realm for security types that require it. A value must be<br>specified if using Kerberos.<br>**Values:**The actual value for the realm.<br>**Default:**kerberos|
|•<br>m = Key Distribution<br>Center (KDC)*|Specifies the KDC for security types that require it. A value must be<br>specified if using Kerberos.<br>**Values:**The actual value for the KDC.<br>**Default:**krbtgt"|
|•<br>`n` = Pre-Shared Key<br>(PSK) value*|Enter the PSK value. This value is calculated and must be the same for each<br>device on the WLAN. Use ZebraNet Bridge to generate the PSK value. A<br>value must be specified if using WPA PSK.<br>**NOTE:** **important:** Do not enter a pass phrase for this field in<br>this command. To use a pass phrase, use the ZebraNet Bridge<br>Enterprise Wireless Setup Wizard.<br>**Values:**a minimum of 64 hexadecimal digits**Default:**None|
|* Not used for all security types|* Not used for all security types|


**Supporting Parameters for Different Security Types**


The supporting parameters required for this command vary based on the security type that you select. You
should not use all of the supporting parameters each time that you use this command, nor will you use


406


ZPL Network Commands


extra commas to separate unused fields. Follow the example and format for your specific security type in
this section, substituting your own wireless network data.


**Security Type 01: No Wireless Security Active**


**Format:** ^WX01


**Example:** This example turns off all wireless securities controlled under this command, but it does not reset
the printer’s wireless settings to their defaults.

```
^XA
^WX01
^JUS^XZ

```

**Security Type 02: WEP 40-Bit**


**Format:** ^WX02,b,c,d,e,f,g,h


**Example:** This example configures the printer for WEP 40-bit encryption using index key 1, open
authentication, and a hexadecimal WEP key with a value of “ `A1B2C3D4F5` .”

```
^XA
^WX02,1,O,H,A1B2C3D4F5,,,
^JUS
^XZ

```

**NOTE:** This is no longer valid for Link OS 6 printers.


**Security Type 03: WEP 128-Bit**


**Format:** ^WX03,b,c,d,e,f,g,h


**Example:** This example configures the printer for WEP 128-bit encryption using index key 2, open
authentication, and four hexadecimal WEP keys.

```
^XA
^WX03,2,O,H,001122334455667788,112233445566778899,223344556677889900,334455667788990
^XZ

```

**NOTE:** This command is not valid for printers running Link OS 6 or later versions.


**Security Type 04: EAP-TLS**


**Format:** ^WX04,k


**Example:** This example configures the printer for EAP-TLS authentication with an optional private key
password with a value of “ `private` .”

```
^XA
^WX04,private
^JUS
^XZ

```

407


ZPL Network Commands


**Security Type 05: EAP-TTLS**


**Format:** ^WX05,i,j

**Example:** This example configures the printer for EAP-TTLS authentication, including a user ID of “ `user` ”
and a password of “ `password` .”

```
^XA
^WX05,user,password
^JUS
^XZ

```

**Security Type 06: EAP-FAST**


**Format:** ^WX06,i,j,k

**Example:** This example configures the printer for EAP-FAST authentication, including a user ID of “ `user`,” a
password of “ `password`,” and an optional private key of “ `private` .”

```
^XA
^WX06,user,password,private
^JUS
^XZ

```

**Security Type 07: PEAP**


**Format:** ^WX07,i,j


**Example:** This example configures the printer for PEAP authentication, including a user ID with a value of
“ `user` ” and a password with a value of “ `password` .”

```
^XA
^WX07,user,password
^JUS
^XZ

```

**Security Type 08: LEAP**


**Format:** ^WX08,i,j


**Example:** This example configures the printer for LEAP authentication, including a user ID with a value of
“ `user` ” and a password with a value of “ `password` .”

```
^XA
^WX08,user,password
^JUS
^XZ

```

**Security Type 09: WPA PSK**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments
(R6x15.x, R53.15.x, ZSPx, and later.)


**Format:** ^WX09,n


408


ZPL Network Commands


**Example:** This example configures the printer for WPA PSK authentication with a PSK value of all zeroes
(64 hexadecimal digits).

```
^XA
^WX09,00000000...^JUS
^XZ

```

**Security Type 10: WPA EAP-TLS**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


**Format:** ^WX10,k


**Example:** This example configures the printer for WPA EAP-TLS authentication with an optional private key
password with a value of “ `private` .”

```
^XA
^WX10,private
^JUS
^XZ

```

**Security Type 11: WPA EAP-TTLS**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


**Format:** ^WX11,i,j


**Example:** This example configures the printer for WPA EAP-TTLS authentication, including a user ID with a
value of “ `user` ” and a password with a value of “ `password` .”

```
^XA
^WX11,user,password
^JUS
^XZ

```

**Security Type 12: WPA EAP-FAST**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


**Format:** ^WX12,i,j,k


**Example:** This example configures the printer for WPA EAP-FAST authentication, including a user ID of
“ `user`,” a password of “ `password`,” and an optional private key of “ `private` .”

```
^XA
^WX12,user,password,private
^JUS
^XZ

```

409


ZPL Network Commands


**Security Type 13: WPA PEAP**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


**Format:** ^WX13,i,j


**Example:** This example configures the printer for WPA PEAP authentication, including a user ID with a
value of “ `user` ” and a password with a value of “ `password` .”

```
^XA
^WX13,user,password
^JUS
^XZ

```

**Security Type 14: WPA LEAP**


**NOTE:** Configuring the printer for WPA also allows the printer to be used in WPA2 environments.


**Format:** ^WX14,i,j


**Example:** This example configures the printer for WPA LEAP authentication, including a user ID with a
value of “ `user` ” and a password with a value of “ `password` .”

```
^XA
^WX14,user,password
^JUS
^XZ

```

**Security Type 15: Kerberos**


**Format:** ^WX15,i,j,l,m


**Example:** This example configures the printer for Kerberos encryption, including a Kerberos user ID with
a value of “ `user`,” a Kerberos password with a value of “ `password`,” a realm of “ `zebra`,” and a KDC of
“ `krbtgt` .”

```
^XA
^WX15,user,password,zebra,krbtgt
^JUS
^XZ

```

410
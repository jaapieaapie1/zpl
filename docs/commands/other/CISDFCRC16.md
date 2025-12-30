# CISDFCRC16 Download Files



The `CISDFCRC16` command downloads supported files types to the printer.


**NOTE:** When using certificate files, your printer supports:


          - Using Privacy Enhanced Mail (PEM) formatted certificate files.


          - Using the client certificate and private key as two files, each downloaded separately.


          - Using exportable PAC files for EAP-FAST.


**NOTE:** When using certificate files, the time on the printer must be set correctly for the
websocket connection to succeed, as the time is used in the certificate validation. Each line
should be terminated with a CR/LF.


**Type**
```
       ! CISDFCRC16
       <crc>
       <filename>
       <size>
       <checksum>
       <data>

|Parameters|Values|
|---|---|
|`<crc>`|A four digit CRC value in hexadecimal. If`0000` is entered, then the CRC<br>validation is ignored. For examples, see below.|
|`<filename>`|File name that is stored on the file system of the printer. An extension must<br>be specified. Files must be saved to the`E:` drive.|
|`<size>`|An eight digit file size specified in hexadecimal which indicates the number<br>of bytes in the`<data>` section.|
|`<checksum>`|A four digit checksum value in hexadecimal. If`0000` is entered, the CRC<br>validation is ignored. The checksum value is calculated using the sum of<br>the bytes in the`<data>` section. For examples, see below.|
|`<data>`|Binary data saved on the printer’s file system as`<filename>`. Number of<br>bytes in this field must match the`<size>` parameter.|


```

**NOTE:** This command can be used in place of the `~DG` and `~DY` command for more saving
and loading options. `~DY` is the preferred command to download TrueType fonts on printers
with firmware later than X.13. The `CISDFCRC16` command also supports downloading wireless
certificate files


**Example 1**

This example shows the `CISDFCRC16` command used to download a private key file ( `privkey.nrd` ) to
the printer. The different sections of the command are on separate lines.

```
       ! CISDFCRC16
       BA0B
       privkey.nrd

```

648


SGD Printer Commands

```
0000037B
E3AF
-----BEGIN RSA PRIVATE KEY----MIICXgIBAAKBgQDQXu/E9YuGlScfWQepZa8Qe/1mJRpmk8oPhPVvam/4M5/WaWQp
3/p1f8J17/hDH8fFq5Dnx3/tHaU7A4SKO8GeghX5hnp/mt4tuQEvsXkCrcgS1puz
z5dbO7ThhuzxYClnr7uiXPvSRXawgwDTPas+0q/6gHeUSXtA0EofuIyv7wIDAQAB
AoGBAJPnf3wn6wT5pE59DJIyakRiLmkt1wKOzvObJfgS7i2Yv1EbeAy9PnPe3vKG
Bovm6A+oi2/qTSTLUTiFc7QHXJPVxLmRiHMbf1Q8j+VJkGTpWt8EY/Px+HSM2HAP
jqd+Im0IiE9RQPsxWQH9UaauF6nl5gIfMF74BIPsVzFXLFfxAkEA6zSrCKCycE/P
14cjZibnLiWxdL3U3I9eWuhmIS37RB6UJFBCWUPWr26HlHzOKqhOUMbFf5hOmvkZ
gciN9A8kxwJBAOLK7Gyorre8iK9IMMWc7OIJc7H8pH1y/N2OtyaC1XuPfqz0H4PH
w2W2m3BhZ7ggHJLLiiFVF+Hr5X7cibFDo5kCQQDFe5lHSzXHWxvViN/N+0gL1RYk
QOcisTW1+n8VyLe5wDr+Km0q6eytq44mvIuWAW6QH/TfZxBIynICKFQX4UctAkAm
P80iAkz9RfnTfhxjp7S35poxoYdodPU6tLAk+ZnhrfDSYJXUFuPYirSqfnMMtbW7
+EICnyRZAP0CqVU7pUm5AkEAnH2O6dKvUvwOEX+CsCVATRrejKLCeJ+6YZWqiD9X
0XGJgrHNXGpDtQiVSGM59p0XnHTZJYjvVNdNOMnhg333nQ==
-----END RSA PRIVATE KEY----
```

**Example 2**


These are examples of CRC and checksum values:

The value of the `<crc>` field is calculated the CRC-16 for the contents of a specified file using the CRC16CCITT polynomial which is x^16 + x^12 + x^5 + 1. It is calculated using an initial CRC of 0x0000.


Given 4 bytes of data : 0x25, 0x62, 0x3F, 0x52:


**1.** Adding all bytes together gives 0x118.


**2.** Drop the carry nibble to get 0x18.


**3.** Get the two's complement of the 0x18 to get 0xE8.


**4.** This is the checksum byte.


649


SGD Printer Commands
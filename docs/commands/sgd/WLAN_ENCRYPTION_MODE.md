# wlan.encryption_mode



This parameter refers to WEP (Wired Equivalent Privacy) encryption. This parameter enables and disables
the printer’s WEP encryption. When using WEP encryption make sure that the encryption key matches the
wireless network WEP encryption key.


**Setvar**

To instruct the printer to turn the LEAP mode `"on"` or `"off":`

```
       ! U1 setvar "wlan.encryption_mode" "value"

```

**Values**

              - `"off"`

              - `"40-bit"`

              - `"128-bit"`

**Default**
```
          "off"

```

**Getvar**


To return the type of encryption that is currently being used by the printer:

```
       ! U1 getvar "wlan.encryption_mode"

```

**Example 1**


This example instructs the printer to set encryption to 40-bit.

```
       ! U1 setvar "wlan.encryption_mode" "40-bit"

```

**Example 2**


This example instructs the printer to turn encryption off.

```
       ! U1 setvar "wlan.encryption_mode" "off"

```

1400




SGD Network Commands
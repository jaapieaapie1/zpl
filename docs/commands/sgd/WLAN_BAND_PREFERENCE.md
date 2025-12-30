# wlan.band_preference



This command sets a preferred band to connect with using Wi-Fi.


**Setvar**


To specify the WLAN band preference:

```
       ! U1 getvar "wlan.band_preference" "none"

```

**Values**

              - `"2.4"`

              - `"5"`

              - `"none"`

**Default**
```
          "none"

```

**Getvar**


To return the current WLAN band preference value:

```
       ! U1 getvar "wlan.band_preference"

```

**Example**

In the `setvar` example below, the WLAN band preference is set to `"5"` .

```
       ! U1 getvar "wlan.band_preference" "5"

```

1390




SGD Network Commands
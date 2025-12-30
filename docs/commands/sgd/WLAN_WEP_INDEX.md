# wlan.wep.index



This command sets the WEP (Wired Equivalent Privacy) encryption key index. This printer setting
determines which one of the four encryption keys is to be used by the client (printer).


**NOTE:** This command is not supported for printers running Link-OS 6 or later versions.


**Setvar**


To set the encryption key index:

```
       ! U1 setvar "wlan.wep.index" "value"

```

**Values**

              - `"1"` enables encryption key 1

              - `"2"` enables encryption key 2

              - `"3"` enables encryption key 3

              - `"4"` enables encryption key 4

**Default**
```
          "1"

```

**Getvar**


To respond with the encryption key index:

```
       ! U1 getvar "wlan.wep.index"

```

**Example**

This `setvar` example shows the value set to `"1"` .

```
       ! U1 setvar "wlan.wep.index" "1"

```

When the `setvar` value is set to `"1"`, the `getvar` result is `"1"` .


1497


SGD Network Commands
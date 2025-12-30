# wlan.encryption_index



This parameter refers to the WEP (Wired Equivalent Privacy) encryption key index. It determines which one
of the four encryption keys is to be used by the client (printer).


**Setvar**


To set the encryption key index to the specified value:

```
       ! U1 setvar "wlan.encryption_index" "value"

```

**Values**

              - `"1"`

              - `"2"`

              - `"3"`

              - `"4"`

**Default**
```
          "1"

```

**Getvar**


To retrieve the current encryption key setting:

```
       ! U1 getvar "wlan.encryption_index"

```

**Example**

In this example, the `setvar` command instructs the printer to set the encryption key index to 1.

```
       ! U1 setvar "encryption_index" "1"

```

1397


SGD Network Commands
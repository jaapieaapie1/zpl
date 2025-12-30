# wlan.11n.greenfield



This command will enable or disable greenfield mode in 802.11n radio communications.


**Setvar**


To enable or disable the greenfield mode:

```
       ! U1 setvar "wlan.11n.greenfield" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "off"

```

**Getvar**


To return the current setting for greenfield mode:

```
       ! U1 getvar "wlan.11n.greenfield"

```

**Example**

```
       ! U1 setvar "wlan.11n.greenfield" "on"

```

1365


SGD Network Commands
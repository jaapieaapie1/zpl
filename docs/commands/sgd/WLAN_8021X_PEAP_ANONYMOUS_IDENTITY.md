# wlan.8021x.peap.anonymous_identity



This command is used to specify the phase 1 ID when using PEAP to authenticate with the wireless
network.


**Setvar**


To specify the phase 1 ID to be used during peap authentication:

```
       ! U1 setvar "wlan.8021x.peap.anonymous_identity" "<value>"

```

**Values**

`"<value>"` is less than or equal to 32 characters

**Default**
```
          ""

```

**Getvar**


To retrieve the current value:

```
       ! U1 getvar "wlan.8021x.peap.anonymous_identity"

```

1371


SGD Network Commands
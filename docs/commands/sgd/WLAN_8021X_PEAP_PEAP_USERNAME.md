# wlan.8021x.peap.peap_username



Sets the user name associated with the PEAP authentication protocol.


**Setvar**


To set the user name associated with the PEAP authentication protocol:

```
       ! U1 getvar "wlan.8021x.peap.peap_username" "username"

```

**Values**


32 characters or less representing the PEAP username.


**Default**
```
          "username"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.8021x.peap.peap_username"

```

**Result**
```
          "username"

```

1378


SGD Network Commands
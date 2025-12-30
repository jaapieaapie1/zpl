# wlan.8021x.eap.username



Sets or returns the user name associated with the EAP authentication protocol.


**Setvar**


To set the EAP user name:

```
       ! U1 setvar "wlan.8021x.eap.username" "username"

```

**Values**


32 characters or less representing the EAP user name.


**Default**
```
          ""

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.8021x.eap.username"

```

**Result**
```
          "username"

```

1374


SGD Network Commands
# wlan.8021x.peap.peap_password



Sets the password associated with the PEAP authentication protocol.


**Setvar**


To set the PEAP password:

```
       ! U1 setvar "wlan.8021x.peap.peap_password" "password"

```

**Values**


32 characters or less representing the PEAP password.


**Default**
```
          "password"

```

**Getvar**


To return the current setting value:

```
       ! U1 setvar "wlan.8021x.peap.peap_password"

```

**Result**
```
          "*"
```

Printer reports one `"*"` regardless of the length of the stored PEAP authentication password, and
does not reveal the actual password.


1376


SGD Network Commands
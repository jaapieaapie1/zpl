# wlan.8021x.peap.privkey_password



Sets or returns the PEAP authentication private key password.


**Setvar**


To set the PEAP authentication password:

```
       ! U1 setvar "wlan.8021x.peap.privkey_password" "password"

```

**Values**


32 characters or less representing the PEAP private key password.


**Default**
```
          "*"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.8021x.peap.privkey_password"

```

**Result**
```
          "*"
```

Printer reports one `"*"` regardless of the length of the stored PEAP private key password, and
does not reveal the actual password.


1377


SGD Network Commands
# wlan.8021x.eap.privkey_password



Sets the EAP private key password to the specified password.


**Setvar**


To set the EAP private key password:

```
       ! U1 setvar "wlan.8021x.eap.privkey_password" "password"

```

**Values**


32 characters or less representing the EAP private key password.


**Default**
```
          ""

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.8021x.eap.privkey_password"

```

**Result**
```
          "*"
```

Printer always retrieves `"*"` regardless of the length of the stored EAP authentication private key
password, and does not reveal the actual password.


1375


SGD Network Commands
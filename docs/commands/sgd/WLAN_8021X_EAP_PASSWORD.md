# wlan.8021x.eap.password



Sets the EAP authentication password.


**Setvar**


To set the EAP authentcation password:

```
       ! U1 setvar "wlan.8021x.eap.password" "password"

```

**Values**


32 characters or less representing the EAP password.


**Default**
```
          ""

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.8021x.eap.password"

```

**Result**
```
          "*"
```

Password is not readable. Printer reports "*" in response to this command.


1373


SGD Network Commands
# internal_wired.8021x.security



Returns the security type used for the wired network.


**Setvar**


To set the command:

```
       ! U1 setvar "internal_wired.8021x.security" "none"

```

**Values**

              - `"none"`

              - `"peap"`

              - `"eap-tls"`

              - `"eap-ttls"`

**Default**
```
          "none"

```

**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "internal_wired.8021x.security"

```

1161


SGD Network Commands
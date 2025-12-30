# internal_wired.8021x.peap.validate_server_certificate



When using PEAP, this command determines if the printer requires the server certificate to be signed by a
CA in Zebra’s CA chain of trust.


**Setvar**


To set the command:

```
       ! U1 setvar "internal_wired.8021x.peap.validate_server_certificate" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "on"

```

**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "internal_wired.8021x.peap.validate_server_certificate"

```

1158


SGD Network Commands
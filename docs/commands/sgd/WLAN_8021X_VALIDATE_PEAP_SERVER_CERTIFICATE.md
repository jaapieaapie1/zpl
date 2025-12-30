# wlan.8021x.validate_peap_server_certificate



This command determines if the printer will validate the PEAP server certificate.


**Setvar**


To determine if the printer will validate the PEAP server certificate:

```
       ! U1 setvar "wlan.8021x.validate_peap_server_certificate" "value"

```

**Values**

              - `"on"`

              - `"off"`

**Default**
```
          "on"

```

**Getvar**


To return the current setting:

```
       ! U1 getvar "wlan.8021x.validate_peap_server_certificate"

```

**Example**

```
       ! U1 setvar "wlan.8021x.validate_peap_server_certificate"

```

**IMPORTANT:** When using certificate files, the time on the printer must be set correctly for the
websocket connection to succeed, as the time is used in the certificate validation.


1370




SGD Network Commands
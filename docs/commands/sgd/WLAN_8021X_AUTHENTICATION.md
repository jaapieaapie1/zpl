# wlan.8021x.authentication



Sets the authentication type used in the 802.1x security protocol.


**Setvar**


To set the authentication type:

```
       ! U1 setvar "wlan.8021x.authentication" "value"

```

**Values**

`"psk"` Pre-Shared Key

`"leap"` Lightweight Extensible Authentication Protocol

`"eap-tls"` EAP-Transport Layer Security

`"peap"` Protected Extensible Authentication Protocol

`"ttls"` Tunneled Transport Layer Security

`"fast"` Flexible Authentication via Secure Tunneling


**Default**
```
          "psk"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "wlan.8021x.authentication"

```

1372


SGD Network Commands
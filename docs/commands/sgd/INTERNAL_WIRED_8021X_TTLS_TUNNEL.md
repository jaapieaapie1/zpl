# internal_wired.8021x.ttls_tunnel



Sets the TTLS tunnel protocol to use in the authentication process.


**Setvar**


To set the command:

```
       ! U1 setvar "internal_wired.8021x.ttls_tunnel" "value"

```

**Values**

              - `"pap"`

              - `"chap"`

              - `"mschap"`

              - `"mschapv2"`

**Default**
```
          "mschapv2"

```

**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "internal_wired.8021x.ttls_tunnel"

```

1163


SGD Network Commands
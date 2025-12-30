# weblink.zebra_connector.enable



Enables the Visibility Agent feature.


**Setvar**


To enable or disable the Visibility Agent feature:

```
       ! U1 setvar "weblink.zebra_connector.enable" "value"

```

**Values**

              - `"on"` enables Visibility Agent

              - `"off` disables Visibility Agent

**Default**
```
          "on"

```

**Example**

```
       ! U1 setvar "weblink.zebra_connector.enable" "enable"

```

**Getvar**


To return the current setting value:

```
       ! U1 getvar "weblink.zebra_connector.enable"

```

**Result**


"on" or "off"


1357


SGD Network Commands
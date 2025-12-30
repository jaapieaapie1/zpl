# ip.https.enable



Enables/disables the HTTPS web connections.


**IMPORTANT:** A network or printer reset is required for this setting to take effect.


**Setvar**


To set the command:

```
       ! U1 setvar "ip.https.enable" "value"

```

**Values**

              - `"off"` disables HTTPS protocol

              - `"on"` enables HTTPS protocol

**Default**

              - **For printers purchased in the EMEA region after August 1, 2025:** `"off"`

              - **For all other printers:** `"on"`


**Getvar**


To have the printer return the current setting value:

```
       ! U1 getvar "ip.https.enable"

```

1262


SGD Network Commands